use serde::Deserialize;

use crate::{
    AccessState, AwattarPriceZone, CableCoding, CarStatus, EnergySensorReading, GoEStatusError,
    PhaseStatus, StopState,
};

// https://github.com/goecharger/go-eCharger-API-v1/blob/master/go-eCharger%20API%20v1%20EN.md
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // for the sake of completeness
pub(crate) struct StatusJson {
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

#[derive(Debug)]
pub struct GoEStatus {
    pub car_status: CarStatus,
    pub ampere: u8,
    pub access_state: AccessState,
    pub allow_charging: bool,
    pub stop_state: StopState,
    pub cable_coding: CableCoding,
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
    pub(crate) fn try_from_status_json(status_json: &StatusJson) -> Result<Self, GoEStatusError> {
        Ok(Self {
            car_status: status_json.car.parse()?,
            ampere: status_json.amp.parse()?,
            access_state: status_json.ast.parse()?,
            allow_charging: status_json.alw.parse::<u8>()? == 1,
            stop_state: status_json.stp.parse()?,
            cable_coding: status_json.cbl.parse()?,
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
}
