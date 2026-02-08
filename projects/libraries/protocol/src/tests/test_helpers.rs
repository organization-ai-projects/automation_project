// projects/libraries/protocol/src/tests/test_helpers.rs
//! Common test helpers for protocol library tests

use crate::ProtocolId;

/// Helper to validate that a ProtocolId has proper hex formatting
pub(crate) fn assert_valid_protocol_id_hex(id: &ProtocolId) {
    let hex = id.to_hex();
    assert_eq!(hex.len(), 32, "Protocol ID should be 32 hex characters");
    assert!(
        hex.chars().all(|c| c.is_ascii_hexdigit()),
        "Protocol ID should be valid hex"
    );
}
