use crate::issues::commands::OpenNumbersOptions;

#[test]
fn open_numbers_options_can_be_built() {
    let value = OpenNumbersOptions {
        repo: Some("owner/repo".to_string()),
    };
    assert_eq!(value.repo.as_deref(), Some("owner/repo"));
}
