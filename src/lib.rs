use std::{
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};

use thiserror::Error;

pub mod connection;
pub mod status;

use crate::connection::ChargerConnection;
use crate::status::GoEStatus;

#[derive(Debug, PartialEq, Eq)]
pub enum CarStatus {
    ReadyNoVehicle,
    Charging,
    WaitingForVehicle,
    ChargingFinished,
}

impl FromStr for CarStatus {
    type Err = GoEStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::ReadyNoVehicle),
            "2" => Ok(Self::Charging),
            "3" => Ok(Self::WaitingForVehicle),
            "4" => Ok(Self::ChargingFinished),
            _ => Err(GoEStatusError::InvalidValue(format!(
                "Invalid car status '{s}'"
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AccessState {
    Open,
    RFID,
    ElectricityPrices,
}

impl FromStr for AccessState {
    type Err = GoEStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Open),
            "1" => Ok(Self::RFID),
            "2" => Ok(Self::ElectricityPrices),
            _ => Err(GoEStatusError::InvalidValue(format!(
                "Invalid access state '{s}'"
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StopState {
    Deactivated,
    SwitchOffAfterKwh,
}

impl FromStr for StopState {
    type Err = GoEStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Deactivated),
            "2" => Ok(Self::SwitchOffAfterKwh),
            _ => Err(GoEStatusError::InvalidValue(format!(
                "Invalid value for stop state '{s}'"
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CableCoding {
    NoCable,
    Ampere(u8),
}

impl FromStr for CableCoding {
    type Err = GoEStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.parse::<u8>()?;

        match num {
            0 => Ok(Self::NoCable),
            13..=32 => Ok(Self::Ampere(num)),
            _ => Err(GoEStatusError::InvalidValue(format!(
                "Cable coding out of range: {num}"
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PhaseStatus {
    pub l1_before_contactor: bool,
    pub l1_after_contactor: bool,

    pub l2_before_contactor: bool,
    pub l2_after_contactor: bool,

    pub l3_before_contactor: bool,
    pub l3_after_contactor: bool,
}

impl From<u8> for PhaseStatus {
    fn from(value: u8) -> Self {
        PhaseStatus {
            l1_before_contactor: value & (1 << 3) > 0,
            l1_after_contactor: value & (1 << 0) > 0,

            l2_before_contactor: value & (1 << 4) > 0,
            l2_after_contactor: value & (1 << 1) > 0,

            l3_before_contactor: value & (1 << 5) > 0,
            l3_after_contactor: value & (1 << 2) > 0,
        }
    }
}

#[derive(Debug)]
pub struct EnergySensorReading {
    pub voltage_l1: i32,
    pub voltage_l2: i32,
    pub voltage_l3: i32,
    pub voltage_n: i32,

    pub current_l1: i32,
    pub current_l2: i32,
    pub current_l3: i32,

    pub power_l1: i32,
    pub power_l2: i32,
    pub power_l3: i32,
    pub power_n: i32,
    pub power_total: i32,

    pub powerfactor_l1: i32,
    pub powerfactor_l2: i32,
    pub powerfactor_l3: i32,
    pub powerfactor_n: i32,
}

impl EnergySensorReading {
    pub fn from_nrg_array(nrg: &[i32; 16]) -> Result<Self, GoEStatusError> {
        Ok(Self {
            voltage_l1: nrg[0],
            voltage_l2: nrg[1],
            voltage_l3: nrg[2],
            voltage_n: nrg[3],

            current_l1: nrg[4],
            current_l2: nrg[5],
            current_l3: nrg[6],

            power_l1: nrg[7],
            power_l2: nrg[8],
            power_l3: nrg[9],
            power_n: nrg[10],
            power_total: nrg[11],

            powerfactor_l1: nrg[12],
            powerfactor_l2: nrg[13],
            powerfactor_l3: nrg[14],
            powerfactor_n: nrg[15],
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AwattarPriceZone {
    Austria,
    Germany,
}

impl FromStr for AwattarPriceZone {
    type Err = GoEStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Austria),
            "1" => Ok(Self::Germany),
            _ => Err(GoEStatusError::InvalidValue(format!(
                "Invalid awattar price zone value '{s}'"
            ))),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GoEStatusError {
    #[error("Parsing int failed")]
    ParseIntError(#[from] ParseIntError),
    #[error("Parsing bool failed")]
    ParseBoolError(#[from] ParseBoolError),
    #[error("Invalid value")]
    InvalidValue(String),
}

pub struct GoECharger<C: ChargerConnection> {
    connection: C,
}

impl<C: ChargerConnection> GoECharger<C> {
    pub fn new(connection: C) -> Self {
        Self { connection }
    }

    pub async fn latest_status(&self) -> Result<GoEStatus, GoEError> {
        self.connection.latest_status().await
    }

    pub async fn set_ampere(&mut self, ampere: u32) -> Result<(), GoEError> {
        self.connection.set_key("amp", &ampere.to_string()).await
    }

    pub async fn set_access_state(&mut self, access_state: AccessState) -> Result<(), GoEError> {
        self.connection
            .set_key(
                "ast",
                match access_state {
                    AccessState::Open => "0",
                    AccessState::RFID => "1",
                    AccessState::ElectricityPrices => "2",
                },
            )
            .await
    }

    pub async fn set_allow_charging(&mut self, allow: bool) -> Result<(), GoEError> {
        self.connection
            .set_key("alw", if allow { "1" } else { "0" })
            .await
    }
}

#[derive(Debug, Error)]
pub enum GoEError {
    #[error("Connection error")]
    HttpError(#[from] reqwest::Error),
    // #[cfg(feature = "mqtt")]
    // MqttError(#[from] ???),
    #[error("Status parsing error")]
    StatusError(#[from] GoEStatusError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_carstatus() {
        assert_eq!(CarStatus::from_str("1"), Ok(CarStatus::ReadyNoVehicle));
        assert_eq!(CarStatus::from_str("2"), Ok(CarStatus::Charging));
        assert_eq!(CarStatus::from_str("3"), Ok(CarStatus::WaitingForVehicle));
        assert_eq!(CarStatus::from_str("4"), Ok(CarStatus::ChargingFinished));

        // these aren't exactly great tests for the error condition
        assert!(CarStatus::from_str("5").is_err());
        assert!(CarStatus::from_str("").is_err());
        assert!(CarStatus::from_str("abc").is_err());
    }

    #[test]
    fn test_parse_cablecoding() {
        assert!(CableCoding::from_str("a").is_err());
        assert_eq!(CableCoding::from_str("0"), Ok(CableCoding::NoCable));
        assert!(CableCoding::from_str("1").is_err());
        assert!(CableCoding::from_str("12").is_err());
        assert_eq!(CableCoding::from_str("13"), Ok(CableCoding::Ampere(13)));
        assert_eq!(CableCoding::from_str("32"), Ok(CableCoding::Ampere(32)));
        assert!(CableCoding::from_str("33").is_err());
    }

    #[test]
    fn test_phase_status() {
        assert_eq!(
            PhaseStatus::from(0),
            PhaseStatus {
                l1_before_contactor: false,
                l1_after_contactor: false,
                l2_before_contactor: false,
                l2_after_contactor: false,
                l3_before_contactor: false,
                l3_after_contactor: false,
            }
        );

        assert_eq!(
            PhaseStatus::from(0b00111000),
            PhaseStatus {
                l1_before_contactor: true,
                l1_after_contactor: false,
                l2_before_contactor: true,
                l2_after_contactor: false,
                l3_before_contactor: true,
                l3_after_contactor: false,
            }
        );

        assert_eq!(
            PhaseStatus::from(0b00000111),
            PhaseStatus {
                l1_before_contactor: false,
                l1_after_contactor: true,
                l2_before_contactor: false,
                l2_after_contactor: true,
                l3_before_contactor: false,
                l3_after_contactor: true,
            }
        );

        assert_eq!(
            PhaseStatus::from(0b00010101),
            PhaseStatus {
                l1_before_contactor: false,
                l1_after_contactor: true,
                l2_before_contactor: true,
                l2_after_contactor: false,
                l3_before_contactor: false,
                l3_after_contactor: true,
            }
        );
    }
}
