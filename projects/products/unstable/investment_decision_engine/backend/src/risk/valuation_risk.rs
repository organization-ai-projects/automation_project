use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValuationRisk {
    pub pe_ratio: Option<f64>,
    pub fcf_yield: Option<f64>,
    pub score: f64,
}

impl ValuationRisk {
    pub fn compute(pe_ratio: Option<f64>, fcf_yield: Option<f64>) -> Self {
        let pe_score = pe_ratio.map(|pe| {
            if pe > 50.0 { 0.9 }
            else if pe > 30.0 { 0.6 }
            else if pe > 15.0 { 0.3 }
            else { 0.1 }
        }).unwrap_or(0.5);

        let fcf_score = fcf_yield.map(|y| {
            if y < 0.01 { 0.8 }
            else if y < 0.03 { 0.5 }
            else if y < 0.06 { 0.2 }
            else { 0.1 }
        }).unwrap_or(0.5);

        let score = pe_score * 0.5 + fcf_score * 0.5;
        Self {
            pe_ratio,
            fcf_yield,
            score,
        }
    }
}
