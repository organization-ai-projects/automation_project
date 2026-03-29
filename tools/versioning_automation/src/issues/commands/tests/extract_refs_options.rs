//! tools/versioning_automation/src/issues/commands/tests/extract_refs_options.rs
use crate::issues::commands::ExtractRefsProfile;
use crate::issues::commands::extract_refs_options::ExtractRefsOptions;

#[test]
fn test_run_extract_refs_with_text() {
    let options = ExtractRefsOptions {
        profile: ExtractRefsProfile::Hook,
        text: Some("Closes #123".to_string()),
        file: None,
    };
    let result = options.run_extract_refs();
    assert_eq!(result, 0);
}

#[test]
fn test_run_extract_refs_with_file() {
    let options = ExtractRefsOptions {
        profile: ExtractRefsProfile::Audit,
        text: None,
        file: Some("test_file.txt".to_string()),
    };
    let result = options.run_extract_refs();
    assert_eq!(result, 1);
}
