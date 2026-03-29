//! tools/versioning_automation/src/pr/commands/tests/pr_generate_description_options.rs
use crate::pr::commands::pr_generate_description_options::PrGenerateDescriptionOptions;

#[test]
fn test_run_generate_description_valid() {
    let options = PrGenerateDescriptionOptions {
        passthrough: vec!["--help".to_string()],
    };
    let result = options.run_generate_description();
    assert_eq!(result, 0);
}

#[test]
fn test_run_generate_description_invalid_option() {
    let options = PrGenerateDescriptionOptions {
        passthrough: vec!["--invalid".to_string()],
    };
    let result = options.run_generate_description();
    assert_ne!(result, 0);
}
