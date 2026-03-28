use crate::issues::required_fields::Key;

#[test]
fn contract_key_for_default_profile_has_issue_prefix() {
    let key = Key::contract_key_for_profile("", Key::RequiredFields);
    assert_eq!(key, "ISSUE_REQUIRED_FIELDS");
}

#[test]
fn contract_key_for_review_profile_has_review_prefix() {
    let key = Key::contract_key_for_profile("review", Key::TitleRegex);
    assert_eq!(key, "ISSUE_REVIEW_TITLE_REGEX");
}
