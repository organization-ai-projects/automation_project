use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderKind {
    MoveFleet,
    AttackFleet,
    DefendSystem,
    OfferTreaty,
    AcceptTreaty,
    RejectTreaty,
    Embargo,
    Invest,
}
