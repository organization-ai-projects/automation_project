//! projects/products/unstable/autonomous_dev_ai/src/parsing/mod.rs
use crate::persistence::ActionOutcomeStats;

pub(crate) fn parse_action_outcome_triplet(value: &str) -> Option<(String, f64, u32)> {
    if let Ok(summary) = common_json::from_str::<ActionOutcomeStats>(value) {
        return Some((
            summary.action.to_string(),
            summary.pass_rate.get(),
            summary.total,
        ));
    }

    let mut parts = value.split(':');
    let action = parts.next()?.to_string();
    let pass_rate = parts.next()?.parse::<f64>().ok()?;
    let total = parts.next()?.parse::<u32>().ok()?;
    Some((action, pass_rate, total))
}
