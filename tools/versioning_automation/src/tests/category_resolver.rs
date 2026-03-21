//! tools/versioning_automation/src/tests/category_resolver.rs
use crate::category_resolver;

#[test]
fn classify_title_maps_known_prefixes() {
    assert_eq!(
        category_resolver::classify_title("fix(tools/versioning_automation): adjust flow"),
        "Bug Fixes"
    );
    assert_eq!(
        category_resolver::classify_title("chore(workspace): sync deps"),
        "Refactoring"
    );
    assert_eq!(
        category_resolver::classify_title("feat(product): add endpoint"),
        "Features"
    );
}

#[test]
fn resolve_issue_outcome_category_falls_back_to_default_when_issue_is_missing() {
    let category = category_resolver::resolve_issue_outcome_category("#__invalid__", "Refactoring");
    assert_eq!(category, "Refactoring");
}
