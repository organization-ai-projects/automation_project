use serde::{Deserialize, Serialize};
use crate::time::turn::Turn;
use crate::adjudication::adjudication_report::AdjudicationReport;
use crate::orders::order_set::OrderSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnReport {
    pub turn: Turn,
    pub order_sets: Vec<OrderSet>,
    pub adjudication: AdjudicationReport,
}
