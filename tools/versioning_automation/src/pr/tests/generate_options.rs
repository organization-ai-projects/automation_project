//! tools/versioning_automation/src/pr/tests/generate_options.rs
use crate::pr::generate_options::GenerateOptions;

#[test]
fn test_parse_generate_options_valid_args() {
    let args = vec![
        "--create-pr".to_string(),
        "--base-ref".to_string(),
        "main".to_string(),
        "--head-ref".to_string(),
        "feature".to_string(),
    ];
    let options = GenerateOptions::parse_generate_options(&args).unwrap();
    assert!(options.create_pr);
    assert_eq!(options.base_ref, Some("main".to_string()));
    assert_eq!(options.head_ref, Some("feature".to_string()));
}

#[test]
fn test_parse_generate_options_invalid_args() {
    let args = vec!["--invalid-flag".to_string()];
    let result = GenerateOptions::parse_generate_options(&args);
    assert!(result.is_err());
}
