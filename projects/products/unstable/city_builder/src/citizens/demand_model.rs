use crate::zoning::ZoneKind;

#[derive(Debug, Clone)]
pub struct DemandModel;

impl DemandModel {
    pub fn demand(zone: ZoneKind) -> u64 {
        match zone {
            ZoneKind::Residential => 100,
            ZoneKind::Commercial => 50,
            ZoneKind::Industrial => 30,
            ZoneKind::None => 0,
        }
    }
}
