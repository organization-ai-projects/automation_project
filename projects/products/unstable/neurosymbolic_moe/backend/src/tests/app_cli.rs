//! projects/products/unstable/neurosymbolic_moe/backend/src/tests/app_cli.rs
use std::sync::atomic::{AtomicU64, Ordering};
use std::{fs, path::PathBuf};

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
    assert!(parsed.4.is_none());
    assert!(parsed.5.is_none());
    assert!(parsed.6.is_none());
    assert!(!parsed.7);
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
    assert!(parsed.4.is_none());
}

#[test]
fn serve_metrics_options_parse_profile_path() {
    let args = vec![
        "--slo-profile-path".to_string(),
        "/tmp/neuro_slo_profile.txt".to_string(),
    ];
    let parsed = crate::app::parse_serve_metrics_options(&args).expect("serve args should parse");
    assert_eq!(parsed.4.as_deref(), Some("/tmp/neuro_slo_profile.txt"));
    assert!(parsed.5.is_none());
    assert!(parsed.6.is_none());
    assert!(!parsed.7);
}

#[test]
fn serve_metrics_options_parse_admin_and_audit_flags() {
    let args = vec![
        "--admin-token".to_string(),
        "secret-token".to_string(),
        "--slo-audit-path".to_string(),
        "/tmp/slo_audit.log".to_string(),
        "--disable-auto-rollback".to_string(),
    ];
    let parsed = crate::app::parse_serve_metrics_options(&args).expect("serve args should parse");
    assert_eq!(parsed.5.as_deref(), Some("secret-token"));
    assert_eq!(parsed.6.as_deref(), Some("/tmp/slo_audit.log"));
    assert!(parsed.7);
}

#[test]
fn admin_profile_query_parser_extracts_profile() {
    let line = "POST /admin/slo-profile?profile=strict HTTP/1.1\r\nHost: x\r\n";
    let parsed = crate::app::parse_admin_profile_from_request_line(line);
    assert_eq!(parsed, Some("strict"));
}

#[test]
fn admin_token_authorization_parses_header_case_insensitive() {
    let line =
        "POST /admin/slo-profile?profile=strict HTTP/1.1\r\nx-admin-token: token-123\r\n\r\n";
    let ok = crate::app::is_authorized_admin_request(line, Some("token-123"));
    let denied = crate::app::is_authorized_admin_request(line, Some("wrong"));
    assert!(ok);
    assert!(!denied);
}

#[test]
fn admin_audit_limit_query_parser_extracts_limit() {
    let line = "GET /admin/slo-audit?limit=120 HTTP/1.1\r\nHost: x\r\n";
    let parsed = crate::app::parse_admin_audit_limit_from_request_line(line);
    assert_eq!(parsed, Some(120));
}

#[test]
fn admin_audit_limit_query_parser_rejects_zero() {
    let line = "GET /admin/slo-audit?limit=0 HTTP/1.1\r\nHost: x\r\n";
    let parsed = crate::app::parse_admin_audit_limit_from_request_line(line);
    assert_eq!(parsed, None);
}

#[test]
fn format_slo_audit_entry_json_escapes_content() {
    let json = crate::app::format_slo_audit_entry_json(
        7,
        "strict",
        "balanced",
        "applied",
        "line1\nline2 \"quoted\"",
    );
    assert!(json.contains("\"seq\":7"));
    assert!(json.contains("\"from_profile\":\"strict\""));
    assert!(json.contains("\"to_profile\":\"balanced\""));
    assert!(json.contains("\"reason\":\"line1 line2 \\\"quoted\\\"\""));
}

#[test]
fn read_admin_audit_json_returns_tail_only() {
    let path = unique_test_file_path("audit_tail");
    let payload = [
        crate::app::format_slo_audit_entry_json(1, "balanced", "strict", "applied", "a"),
        crate::app::format_slo_audit_entry_json(2, "strict", "exploratory", "rejected", "b"),
        crate::app::format_slo_audit_entry_json(3, "strict", "balanced", "applied", "c"),
    ]
    .join("\n");
    fs::write(&path, format!("{payload}\n")).expect("write audit fixture");

    let result =
        crate::app::read_admin_audit_json(path.to_str().expect("utf8 path"), 2).expect("read");
    assert!(!result.contains("\"seq\":1"));
    assert!(result.contains("\"seq\":2"));
    assert!(result.contains("\"seq\":3"));

    let _ = fs::remove_file(path);
}

fn unique_test_file_path(prefix: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let suffix = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "neurosymbolic_moe_{prefix}_{}_{}.jsonl",
        std::process::id(),
        suffix
    ))
}
