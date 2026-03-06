use serde::{Deserialize, Serialize};

use crate::model::empire_id::EmpireId;

use super::treaty_kind::TreatyKind;

/// A pending treaty offer from one empire to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyOffer {
    pub from: EmpireId,
    pub to: EmpireId,
    pub kind: TreatyKind,
    pub proposed_end_turn: Option<u64>,
    pub offer_turn: u64,
}
