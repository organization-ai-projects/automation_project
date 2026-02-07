// Test helpers for security tests
use common::custom_uuid::Id128;
use common_time::timestamp_utils::current_timestamp_ms;
use protocol::ProtocolId;

use crate::{Role, Token};

/// Helper to create a ProtocolId from a single byte pattern
pub fn test_protocol_id(byte: u8) -> ProtocolId {
    ProtocolId::new(Id128::from_bytes_unchecked([byte; 16]))
}

/// Helper to build a token with specified parameters
pub fn build_test_token(role: Role, subject_id: ProtocolId, expires_at_ms: u64) -> Token {
    let issued_at_ms = current_timestamp_ms();
    Token {
        value: test_protocol_id(7),
        subject_id,
        role,
        issued_at_ms,
        expires_at_ms,
        session_id: None,
    }
}

/// Helper to build a token that expires after a given number of milliseconds
pub fn build_token_expires_in(role: Role, subject_id: ProtocolId, duration_ms: u64) -> Token {
    build_test_token(
        role,
        subject_id,
        current_timestamp_ms().saturating_add(duration_ms),
    )
}

/// Helper to build an already expired token
pub fn build_expired_token(role: Role, subject_id: ProtocolId) -> Token {
    build_test_token(role, subject_id, current_timestamp_ms().saturating_sub(1))
}
