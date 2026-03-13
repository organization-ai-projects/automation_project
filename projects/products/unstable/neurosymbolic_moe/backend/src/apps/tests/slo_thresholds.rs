//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/tests/slo_thresholds.rs

use crate::apps::SloThresholds;

#[test]
fn slo_thresholds_parse_args_overrides_defaults() {
    let args = vec![
        "--runtime-min-successes".to_string(),
        "3".to_string(),
        "--concurrent-max-timeout-rate".to_string(),
        "0.15".to_string(),
    ];
    let parsed = SloThresholds::parse_args(&args).expect("threshold args should parse");
    assert_eq!(parsed.runtime_min_successes, 3);
    assert!((parsed.concurrent_max_timeout_rate - 0.15).abs() < f64::EPSILON);
}
