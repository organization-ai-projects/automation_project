use common::custom_uuid::Id128;
use protocol::ProtocolId;

use common_time::timestamp_utils::current_timestamp_ms;

use crate::{Role, Token, TokenError};

fn build_token(role: Role, subject_id: ProtocolId, expires_at_ms: u64) -> Token {
    let issued_at_ms = current_timestamp_ms();
    Token {
        value: ProtocolId::new(Id128::from_bytes_unchecked([7u8; 16])),
        subject_id,
        role,
        issued_at_ms,
        expires_at_ms,
        session_id: None,
    }
}

#[test]
fn test_is_expired() {
    let id = ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16]));
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_sub(1));
    assert!(token.is_expired());
}

#[test]
fn test_is_expired_with_grace() {
    let id = ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16]));
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
    let id = ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16]));
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_add(5000));
    let remaining = token.time_until_expiry_ms();
    assert!(remaining > 4500 && remaining <= 5000);
}

#[test]
fn test_validate_token() {
    let id_valid = ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16]));
    let valid = build_token(
        Role::User,
        id_valid,
        current_timestamp_ms().saturating_add(5000),
    );
    assert!(valid.validate_token().is_ok());

    let id_expired = ProtocolId::new(Id128::from_bytes_unchecked([2u8; 16]));
    let expired = build_token(
        Role::User,
        id_expired,
        current_timestamp_ms().saturating_sub(1),
    );
    assert!(matches!(expired.validate_token(), Err(TokenError::Expired)));
}

#[test]
fn test_token_creation() {
    let id = ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16]));
    let token = build_token(
        Role::User,
        id,
        current_timestamp_ms().saturating_add(3600000),
    );
    assert_eq!(token.subject_id, id);
}

#[test]
fn test_token_with_session() {
    let id = ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16]));
    let issued_at_ms = current_timestamp_ms();
    let token = Token {
        value: ProtocolId::new(Id128::from_bytes_unchecked([7u8; 16])),
        subject_id: id,
        role: Role::Admin,
        issued_at_ms,
        expires_at_ms: issued_at_ms.saturating_add(3600000),
        session_id: Some(ProtocolId::new(Id128::from_bytes_unchecked([9u8; 16]))),
    };
    assert_eq!(
        token.session_id,
        Some(ProtocolId::new(Id128::from_bytes_unchecked([9u8; 16])))
    );
}

#[test]
fn test_token_expired() {
    let id = ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16]));
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_sub(1));
    assert!(token.is_expired());
}

#[test]
fn test_validate_token_valid() {
    let id = ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16]));
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_add(5000));
    assert!(token.validate_token().is_ok());
}

#[test]
fn test_validate_token_invalid() {
    let id = ProtocolId::new(Id128::from_bytes_unchecked([0u8; 16]));
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_add(5000));
    assert!(token.validate_token().is_err());
}

#[test]
fn test_token_creation_with_id128() {
    let id = ProtocolId::new(Id128::from_bytes_unchecked([123u8; 16]));
    let token = build_token(Role::User, id, current_timestamp_ms().saturating_add(5000));
    assert_eq!(token.subject_id, id);
}
