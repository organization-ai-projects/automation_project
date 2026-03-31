use serde::{Deserialize, Serialize};

use crate::risk::{ConcentrationRisk, DrawdownRisk, ThesisBreakRisk, ValuationRisk};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskScore {
    pub drawdown: DrawdownRisk,
    pub concentration: ConcentrationRisk,
    pub valuation: ValuationRisk,
    pub thesis_break: ThesisBreakRisk,
    pub composite_score: f64,
}

impl RiskScore {
    pub fn compute(
        drawdown: DrawdownRisk,
        concentration: ConcentrationRisk,
        valuation: ValuationRisk,
        thesis_break: ThesisBreakRisk,
    ) -> Self {
        let composite = drawdown.score * 0.25
            + concentration.score * 0.20
            + valuation.score * 0.25
            + thesis_break.score * 0.30;
        Self {
            drawdown,
            concentration,
            valuation,
            thesis_break,
            composite_score: composite,
        }
    }

    pub fn is_high_risk(&self) -> bool {
        self.composite_score > 0.7
    }
}
