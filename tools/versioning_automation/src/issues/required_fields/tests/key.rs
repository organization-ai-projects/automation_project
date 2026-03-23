#[test]
fn contract_key_for_default_profile_has_issue_prefix() {
    let key = crate::issues::required_fields::contract_key_for_profile(
        "",
        crate::issues::required_fields::key::Key::RequiredFields,
    );
    assert_eq!(key, "ISSUE_REQUIRED_FIELDS");
}

#[test]
fn contract_key_for_review_profile_has_review_prefix() {
    let key = crate::issues::required_fields::contract_key_for_profile(
        "review",
        crate::issues::required_fields::key::Key::TitleRegex,
    );
    assert_eq!(key, "ISSUE_REVIEW_TITLE_REGEX");
}
