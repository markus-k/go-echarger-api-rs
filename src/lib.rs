use std::{
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};

use serde::Deserialize;
use thiserror::Error;

pub mod connection;

// https://github.com/goecharger/go-eCharger-API-v1/blob/master/go-eCharger%20API%20v1%20EN.md
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // for the sake of completeness
struct StatusJson {
    version: String,
    tme: String,
    rbc: String,
    rbt: String,
    car: String,
    amp: String,
    err: String,
    ast: String,
    alw: String,
    stp: String,
    cbl: String,
    pha: String,
    tmp: String,
    dws: String,
    dwo: String,
    adi: String,
    uby: String,
    eto: String,
    wst: String,
    txi: String,
    nrg: [i32; 16], // exact type isn't documented
    fwv: String,
    sse: String,
    wss: String,
    wke: String,
    wen: String,
    cdi: String,
    tof: String,
    tds: String,
    lbr: String,
    aho: String,
    afi: String,
    azo: String,
    ama: String,
    al1: String,
    al2: String,
    al3: String,
    al4: String,
    al5: String,
    cid: String,
    cch: String,
    cfi: String,
    lse: String,
    ust: String,
    wak: String,
    r1x: String,
    dto: String,
    nmo: String,
    sch: String,
    sdp: String,
    eca: String,
    ecr: String,
    ecd: String,
    ec4: String,
    ec5: String,
    ec6: String,
    ec7: String,
    ec8: String,
    ec9: String,
    ec1: String,
    rca: String,
    rcr: String,
    rcd: String,
    rc4: String,
    rc5: String,
    rc6: String,
    rc7: String,
    rc8: String,
    rc9: String,
    rc1: String,
    rna: String,
    rnm: String,
    rne: String,
    rn4: String,
    rn5: String,
    rn6: String,
    rn7: String,
    rn8: String,
    rn9: String,
    rn1: String,
    loe: u8,
    lot: u8,
    lom: u8,
    lop: u8,
    log: String,
    lon: u8,
    lof: u8,
    loa: u8,
    lch: u32,
    mce: u8,
    mcs: String,
    mcp: u16,
    mcu: String,
    mck: String,
    mcc: u8,
}

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

#[derive(Debug)]
pub struct GoEStatus {
    pub car_status: CarStatus,
    pub ampere: u8,
    pub allow_charging: bool,
    pub stop_state: StopState,
    pub phase_status: PhaseStatus,
    pub temperature: u8,
    pub charged: u32,
    pub stop_energy: u32,
    pub total_energy: u32,
    pub energy_sensor: EnergySensorReading,
    pub serial_number: String,
    pub awattar_price_zone: AwattarPriceZone,
}

impl GoEStatus {
    fn try_from_status_json(status_json: &StatusJson) -> Result<Self, GoEStatusError> {
        Ok(Self {
            car_status: status_json.car.parse()?,
            ampere: status_json.amp.parse()?,
            allow_charging: status_json.alw.parse::<u8>()? == 1,
            stop_state: status_json.stp.parse()?,
            phase_status: PhaseStatus::from(status_json.pha.parse::<u8>()?),
            temperature: status_json.tmp.parse()?,
            charged: status_json.dws.parse()?,
            stop_energy: status_json.dwo.parse()?,
            total_energy: status_json.eto.parse()?,
            energy_sensor: EnergySensorReading::from_nrg_array(&status_json.nrg)?,
            serial_number: status_json.sse.clone(),
            awattar_price_zone: status_json.azo.parse()?,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_json() {
        let test_data = include_str!("../tests/status.json");

        let status_json: StatusJson = serde_json::from_str(test_data).unwrap();

        // just a smoke test for now
        let _goe_status = GoEStatus::try_from_status_json(&status_json).unwrap();
    }

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
