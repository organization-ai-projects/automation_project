#[test]
fn resolve_repo_name_optional_prefers_explicit_repo() {
    let repo_name = crate::repo_name::resolve_repo_name_optional(Some("owner/repo"));
    assert_eq!(repo_name, Some("owner/repo".to_string()));
}
