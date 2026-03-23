use std::collections::BTreeSet;

use crate::permission_picker::PERMISSION_OPTIONS;

#[test]
fn permission_options_are_unique_and_non_empty() {
    assert!(!PERMISSION_OPTIONS.is_empty());
    let unique: BTreeSet<&str> = PERMISSION_OPTIONS.into_iter().collect();
    assert_eq!(unique.len(), PERMISSION_OPTIONS.len());
    assert!(unique.contains("admin"));
    assert!(unique.contains("configure_system"));
}
