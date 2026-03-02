use serde::{Deserialize, Serialize};

use crate::model::empire_id::EmpireId;

use super::treaty_id::TreatyId;
use super::treaty_kind::TreatyKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Treaty {
    pub id: TreatyId,
    pub kind: TreatyKind,
    pub parties: Vec<EmpireId>,
    pub start_turn: u64,
    pub end_turn: Option<u64>,
    /// Extra treaty rules as key-value pairs.
    pub rules: std::collections::BTreeMap<String, String>,
}
