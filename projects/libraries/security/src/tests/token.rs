use common::custom_uuid::Id128;

use common_time::timestamp_utils::current_timestamp_ms;

use crate::{Role, Token, TokenError};

fn build_token(role: Role, subject_id: Id128, expires_at_ms: u64) -> Token {
    let issued_at_ms = current_timestamp_ms();
    Token {
        value: "test_value".to_string(),
        subject_id,
        role,
        issued_at_ms,
        expires_at_ms,
        session_id: None,
    }
}

#[test]
fn test_is_expired() {
    let id = Id128::from_bytes_unchecked([1u8; 16]);
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_sub(1));
    assert!(token.is_expired());
}

#[test]
fn test_is_expired_with_grace() {
    let id = Id128::from_bytes_unchecked([1u8; 16]);
    let now = current_timestamp_ms();
    let token = build_token(Role::User, id, now.saturating_add(50));
    std::thread::sleep(std::time::Duration::from_millis(60));

    // Expired without grace
    assert!(token.is_expired());

    // Not expired with 100ms grace
    assert!(!token.is_expired_with_grace(100));
}

#[test]
fn test_time_until_expiry() {
    let id = Id128::from_bytes_unchecked([1u8; 16]);
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_add(5000));
    let remaining = token.time_until_expiry_ms();
    assert!(remaining > 4500 && remaining <= 5000);
}

#[test]
fn test_validate_token() {
    let id_valid = Id128::from_bytes_unchecked([1u8; 16]);
    let valid = build_token(
        Role::User,
        id_valid,
        current_timestamp_ms().saturating_add(5000),
    );
    assert!(valid.validate_token().is_ok());

    let id_expired = Id128::from_bytes_unchecked([2u8; 16]);
    let expired = build_token(
        Role::User,
        id_expired,
        current_timestamp_ms().saturating_sub(1),
    );
    assert!(matches!(expired.validate_token(), Err(TokenError::Expired)));
}

#[test]
fn test_token_creation() {
    let id = Id128::from_bytes_unchecked([1u8; 16]);
    let token = build_token(
        Role::User,
        id,
        current_timestamp_ms().saturating_add(3600000),
    );
    assert_eq!(token.subject_id, id);
}

#[test]
fn test_token_with_session() {
    let id = Id128::from_bytes_unchecked([1u8; 16]);
    let issued_at_ms = current_timestamp_ms();
    let token = Token {
        value: "test_value".to_string(),
        subject_id: id,
        role: Role::Admin,
        issued_at_ms,
        expires_at_ms: issued_at_ms.saturating_add(3600000),
        session_id: Some("session_xyz".to_string()),
    };
    assert_eq!(token.session_id.as_deref(), Some("session_xyz"));
}

#[test]
fn test_token_expired() {
    let id = Id128::from_bytes_unchecked([1u8; 16]);
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_sub(1));
    assert!(token.is_expired());
}

#[test]
fn test_validate_token_valid() {
    let id = Id128::from_bytes_unchecked([1u8; 16]);
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_add(5000));
    assert!(token.validate_token().is_ok());
}

#[test]
fn test_validate_token_invalid() {
    let id = Id128::from_bytes_unchecked([0u8; 16]);
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_add(5000));
    assert!(token.validate_token().is_err());
}

#[test]
fn test_token_creation_with_id128() {
    let id = Id128::from_bytes_unchecked([123u8; 16]);
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_add(5000));
    assert_eq!(token.subject_id, id);
}
