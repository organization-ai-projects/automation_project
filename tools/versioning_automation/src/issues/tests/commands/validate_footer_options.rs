use crate::issues::commands::ValidateFooterOptions;

#[test]
fn validate_footer_options_can_be_built() {
    let options = ValidateFooterOptions {
        file: ".git/COMMIT_EDITMSG".to_string(),
        repo: Some("owner/repo".to_string()),
    };
    assert_eq!(options.file, ".git/COMMIT_EDITMSG");
    assert_eq!(options.repo.as_deref(), Some("owner/repo"));
}
