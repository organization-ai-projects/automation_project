use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ResolutionStep {
    ValidateOrders,
    ApplyDiplomacy,
    ResolveCombat,
    UpdateEconomy,
    EmitEvents,
}
