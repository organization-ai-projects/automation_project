use crate::ships::ShipKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatRound {
    pub round_number: u32,
    pub attacker_losses: BTreeMap<ShipKind, u32>,
    pub defender_losses: BTreeMap<ShipKind, u32>,
}
