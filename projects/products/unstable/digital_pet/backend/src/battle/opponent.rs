// projects/products/unstable/digital_pet/backend/src/battle/opponent.rs
use crate::battle::opponent_id::OpponentId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opponent {
    pub id: OpponentId,
    pub name: String,
    pub hp: u32,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
}

impl Opponent {
    pub fn default_opponent() -> Self {
        Self {
            id: OpponentId("opp_1".into()),
            name: "WildMon".into(),
            hp: 25,
            max_hp: 25,
            attack: 8,
            defense: 5,
        }
    }
}
