use common::custom_uuid::Id128;
use protocol::ProtocolId;

use common_time::timestamp_utils::current_timestamp_ms;

use crate::{Role, Token, TokenError};

use super::helpers::{
    build_expired_token, build_test_token, build_token_expires_in, test_protocol_id,
};

#[test]
fn test_is_expired() {
    let id = test_protocol_id(1);
    let token = build_expired_token(Role::User, id);
    assert!(token.is_expired());
}

#[test]
fn test_is_expired_with_grace() {
    let id = test_protocol_id(1);

    // Keep wide margins to avoid timing flakiness on contended CI runners.
    let now = current_timestamp_ms();
    let recently_expired = build_test_token(Role::User, id, now.saturating_sub(1_000));

    // Should be expired without grace
    assert!(recently_expired.is_expired());

    // Should NOT be expired with 2s grace (1s expired + 2s grace = still valid).
    assert!(!recently_expired.is_expired_with_grace(2_000));

    // Should be expired with only 100ms grace (1s ago > 100ms grace).
    assert!(recently_expired.is_expired_with_grace(100));
}

#[test]
fn test_time_until_expiry() {
    let id = test_protocol_id(1);
    let token = build_token_expires_in(Role::User, id, 5000);
    let remaining = token.time_until_expiry_ms();
    assert!(remaining > 4500 && remaining <= 5000);
}

#[test]
fn test_validate_token() {
    let id_valid = test_protocol_id(1);
    let valid = build_token_expires_in(Role::User, id_valid, 5000);
    assert!(valid.validate_token().is_ok());

    let id_expired = test_protocol_id(2);
    let expired = build_expired_token(Role::User, id_expired);
    assert!(matches!(expired.validate_token(), Err(TokenError::Expired)));

    // Test invalid token with zero ID
    let zero_id = ProtocolId::new(Id128::from_bytes_unchecked([0u8; 16]));
    let invalid = build_token_expires_in(Role::User, zero_id, 5000);
    assert!(matches!(
        invalid.validate_token(),
        Err(TokenError::InvalidToken)
    ));
}

#[test]
fn test_token_creation() {
    let id = test_protocol_id(1);
    let token = build_token_expires_in(Role::User, id, 3600000);
    assert_eq!(token.subject_id, id);
}

#[test]
fn test_token_with_session() {
    let id = test_protocol_id(1);
    let issued_at_ms = current_timestamp_ms();
    let token = Token {
        value: test_protocol_id(7),
        subject_id: id,
        role: Role::Admin,
        issued_at_ms,
        expires_at_ms: issued_at_ms.saturating_add(3600000),
        session_id: Some(test_protocol_id(9)),
    };
    assert_eq!(token.session_id, Some(test_protocol_id(9)));
}

#[test]
fn test_token_creation_with_id128() {
    let id = test_protocol_id(123);
    let token = build_token_expires_in(Role::User, id, 5000);
    assert_eq!(token.subject_id, id);
}
