use common_time::timestamp_utils::current_timestamp_ms;

use crate::{
    Permission, PermissionError, Role, Token, check_permission, check_token_permission,
    filter_allowed_permissions, has_all_permissions, has_any_permission, has_permission,
    missing_permissions,
};

use super::helpers::test_protocol_id;

#[test]
fn test_has_permission() {
    assert!(has_permission(&Role::Admin, Permission::Admin));
    assert!(has_permission(&Role::User, Permission::Write));
    assert!(!has_permission(&Role::Guest, Permission::Write));
}

#[test]
fn test_has_all_permissions() {
    let perms = vec![Permission::Read, Permission::Write];
    assert!(has_all_permissions(&Role::User, &perms));
    assert!(!has_all_permissions(&Role::Guest, &perms));
}

#[test]
fn test_has_any_permission() {
    let perms = vec![Permission::Write, Permission::Delete];
    assert!(has_any_permission(&Role::User, &perms)); // Has Write
    assert!(!has_any_permission(&Role::Guest, &perms)); // Has neither
}

#[test]
fn test_check_permission_ok() {
    assert!(check_permission(&Role::User, Permission::Write).is_ok());
}

#[test]
fn test_check_permission_unauthorized() {
    let result = check_permission(&Role::Guest, Permission::Write);
    assert!(result.is_err());
    assert!(matches!(result, Err(PermissionError::Unauthorized)));
}

#[test]
fn test_filter_allowed_permissions() {
    let all_perms = vec![
        Permission::Read,
        Permission::Write,
        Permission::Delete,
        Permission::Admin,
    ];

    let user_allowed = filter_allowed_permissions(&Role::User, &all_perms);
    assert_eq!(user_allowed.len(), 2); // Read, Write
    assert!(user_allowed.contains(&Permission::Read));
    assert!(user_allowed.contains(&Permission::Write));
    assert!(!user_allowed.contains(&Permission::Delete));
}

#[test]
fn test_missing_permissions() {
    let required = vec![Permission::Read, Permission::Write, Permission::Delete];
    let missing = missing_permissions(&Role::User, &required);

    assert_eq!(missing.len(), 1);
    assert!(missing.contains(&Permission::Delete));
}

#[test]
fn test_token_permission() {
    // Create a token with a valid subject id
    let subject_id = test_protocol_id(123);
    let issued_at_ms = current_timestamp_ms();
    let token = Token {
        value: test_protocol_id(7),
        subject_id,
        role: Role::User,
        issued_at_ms,
        expires_at_ms: issued_at_ms.saturating_add(3_600_000),
        session_id: None,
    };

    // Check permissions via the token
    assert!(check_token_permission(&token, Permission::Write).is_ok());
    assert!(check_token_permission(&token, Permission::Delete).is_err());
}
