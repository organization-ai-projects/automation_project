use crate::zoning::ZoneKind;

#[derive(Debug, Clone)]
pub struct TaxPolicy;

impl TaxPolicy {
    pub fn tax_per_building(zone: ZoneKind) -> i64 {
        match zone {
            ZoneKind::Residential => 100,
            ZoneKind::Commercial => 200,
            ZoneKind::Industrial => 150,
            ZoneKind::None => 0,
        }
    }
}
