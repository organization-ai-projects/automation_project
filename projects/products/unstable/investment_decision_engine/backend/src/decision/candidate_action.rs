use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CandidateAction {
    Sell,
    Hold,
    BuyMore,
}

impl CandidateAction {
    pub fn canonical_order() -> Vec<CandidateAction> {
        vec![
            CandidateAction::Sell,
            CandidateAction::Hold,
            CandidateAction::BuyMore,
        ]
    }
}
