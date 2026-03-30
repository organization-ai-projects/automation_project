//! tools/versioning_automation/src/pr/tests/generate_options.rs
use crate::pr::generate_options::GenerateOptions;

#[test]
fn test_parse_generate_options_invalid_args() {
    let args = vec!["--invalid-flag".to_string()];
    let result = GenerateOptions::parse_generate_options(&args);
    assert!(result.is_err());
}
