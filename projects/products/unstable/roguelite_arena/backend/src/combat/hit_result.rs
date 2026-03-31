use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum HitResult {
    Damage { amount: u32 },
    Miss,
    Critical { amount: u32 },
}

impl HitResult {
    pub(crate) fn damage_dealt(&self) -> u32 {
        match self {
            HitResult::Damage { amount } | HitResult::Critical { amount } => *amount,
            HitResult::Miss => 0,
        }
    }
}
