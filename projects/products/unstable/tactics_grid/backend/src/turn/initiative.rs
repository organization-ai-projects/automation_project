use crate::unit::unit::Unit;
use crate::unit::unit_id::UnitId;

#[derive(Debug, Clone)]
pub struct InitiativeEntry {
    pub unit_id: UnitId,
    pub speed: i32,
    pub tie_break: u32,
}

pub struct Initiative;

impl Initiative {
    /// Compute deterministic turn order.
    /// Primary sort: descending speed.
    /// Tie-break: ascending unit id (lower id goes first).
    pub fn compute_order(units: &[Unit]) -> Vec<UnitId> {
        let mut entries: Vec<InitiativeEntry> = units
            .iter()
            .filter(|u| u.alive)
            .map(|u| InitiativeEntry {
                unit_id: u.id,
                speed: u.speed,
                tie_break: u.id.0,
            })
            .collect();

        entries.sort_by(|a, b| {
            b.speed
                .cmp(&a.speed)
                .then_with(|| a.tie_break.cmp(&b.tie_break))
        });

        entries.into_iter().map(|e| e.unit_id).collect()
    }
}
