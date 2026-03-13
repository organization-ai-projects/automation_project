//! projects/products/unstable/neurosymbolic_moe/backend/src/tests/app_cli.rs

#[test]
fn slo_thresholds_parse_args_overrides_defaults() {
    let args = vec![
        "--runtime-min-successes".to_string(),
        "2".to_string(),
        "--concurrent-max-timeout-rate".to_string(),
        "0.05".to_string(),
    ];
    let parsed = crate::apps::SloThresholds::parse_args(&args).expect("args should parse");
    assert_eq!(parsed.runtime_min_successes, 2);
    assert!((parsed.concurrent_max_timeout_rate - 0.05).abs() < f64::EPSILON);
}

#[test]
fn serve_metrics_options_parse_once_and_addr() {
    let args = vec![
        "0.0.0.0:9090".to_string(),
        "--once".to_string(),
        "--cache-ttl-requests".to_string(),
        "25".to_string(),
    ];
    let parsed = crate::app::parse_serve_metrics_options(&args).expect("serve args should parse");
    assert_eq!(parsed.0, "0.0.0.0:9090");
    assert!(parsed.1);
    assert_eq!(parsed.2, 25);
    assert!(parsed.3.is_empty());
}

#[test]
fn serve_metrics_options_forward_threshold_flags() {
    let args = vec![
        "--runtime-min-successes".to_string(),
        "3".to_string(),
        "--profile".to_string(),
        "strict".to_string(),
    ];
    let parsed = crate::app::parse_serve_metrics_options(&args).expect("serve args should parse");
    assert_eq!(parsed.0, "127.0.0.1:9464");
    assert!(!parsed.1);
    assert_eq!(parsed.3.len(), 4);
    assert_eq!(parsed.3[0], "--runtime-min-successes");
    assert_eq!(parsed.3[1], "3");
}
