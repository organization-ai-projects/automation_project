//! projects/products/unstable/autonomous_dev_ai/src/parsing/risk.rs
use crate::lifecycle::ActionRiskLevel;

pub(crate) fn parse_risk_level(value: &str) -> Option<ActionRiskLevel> {
    match value.to_ascii_lowercase().as_str() {
        "low" => Some(ActionRiskLevel::Low),
        "medium" => Some(ActionRiskLevel::Medium),
        "high" => Some(ActionRiskLevel::High),
        _ => None,
    }
}
