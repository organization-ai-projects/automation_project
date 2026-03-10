use crate::components::form_components::{ROLE_OPTIONS, STATUS_OPTIONS};

#[test]
fn role_options_match_supported_roles() {
    assert_eq!(ROLE_OPTIONS, ["admin", "moderator", "user", "guest"]);
}

#[test]
fn status_options_match_supported_statuses() {
    assert_eq!(STATUS_OPTIONS, ["active", "suspended", "disabled"]);
}
