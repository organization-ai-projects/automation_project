#[test]
fn load_contract_values_for_default_profile() {
    let values = crate::issues::required_fields::contract_values::ContractValues::load("")
        .expect("contract values should load from repository config");
    assert_eq!(values.title_regex_key, "ISSUE_TITLE_REGEX");
    assert!(!values.title_regex.trim().is_empty());
    assert!(!values.required_sections.trim().is_empty());
    assert!(!values.required_fields.trim().is_empty());
}

#[test]
fn load_contract_values_for_review_profile() {
    let values = crate::issues::required_fields::contract_values::ContractValues::load("review")
        .expect("review contract values should load from repository config");
    assert_eq!(values.title_regex_key, "ISSUE_REVIEW_TITLE_REGEX");
    assert!(!values.title_regex.trim().is_empty());
}
