use crate::scenario::scenario_result::{CaseResult, ScenarioResult};

#[test]
fn expected_value_is_sum_of_weighted_returns() {
    let cases = vec![
        CaseResult {
            label: "Bull".to_string(),
            projected_price: 130.0,
            projected_return_pct: 0.3,
            probability: 0.4,
            weighted_return: 0.12,
        },
        CaseResult {
            label: "Bear".to_string(),
            projected_price: 70.0,
            projected_return_pct: -0.3,
            probability: 0.6,
            weighted_return: -0.18,
        },
    ];
    let result = ScenarioResult::new("Test", cases);
    assert!((result.expected_value - (-0.06)).abs() < 1e-10);
}
