use serde::{Deserialize, Serialize};

use crate::history::thesis_change::ThesisDirection;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThesisBreakRisk {
    pub latest_direction: ThesisDirection,
    pub score: f64,
}

impl ThesisBreakRisk {
    pub fn compute(latest_direction: ThesisDirection) -> Self {
        let score = match latest_direction {
            ThesisDirection::Broken => 1.0,
            ThesisDirection::Weakened => 0.6,
            ThesisDirection::Unchanged => 0.2,
            ThesisDirection::Strengthened => 0.0,
        };
        Self {
            latest_direction,
            score,
        }
    }
}
