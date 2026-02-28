use super::ZoneKind;

#[derive(Debug, Clone)]
pub struct ZoneRules;

impl ZoneRules {
    pub fn growth_threshold(kind: ZoneKind, tick: u64) -> u64 {
        let base = match kind {
            ZoneKind::Residential => 1,
            ZoneKind::Commercial => 2,
            ZoneKind::Industrial => 3,
            ZoneKind::None => u64::MAX,
        };
        base.max(tick % 3 + 1)
    }
}
