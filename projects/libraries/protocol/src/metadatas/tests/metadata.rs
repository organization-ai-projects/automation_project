// projects/libraries/protocol/src/metadatas/tests/metadata.rs
use crate::{Metadata, ProtocolId, ValidationError};

/// Helper to validate that a ProtocolId has proper hex formatting
fn assert_valid_protocol_id_hex(id: &ProtocolId) {
    let hex = id.to_hex();
    assert_eq!(hex.len(), 32, "Protocol ID should be 32 hex characters");
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()), "Protocol ID should be valid hex");
}

#[test]
fn test_metadata_validate_future_timestamp_rejected() {
    let now = Metadata::current_timestamp_ms();
    let drift_buffer_ms = 10_000;
    let metadata = Metadata {
        request_id: ProtocolId::default(),
        timestamp_ms: Some(now + (3600 * 1000) + drift_buffer_ms),
        ..Default::default()
    };

    match metadata.validate() {
        Ok(_) => panic!("Expected validation error for future timestamp"),
        Err(err) => assert!(matches!(err, ValidationError::InvalidTimestamp(_))),
    }
}

#[test]
fn test_metadata_now_generates_request_id() {
    // Create two metadata instances to verify uniqueness
    let metadata1 = Metadata::now();
    let metadata2 = Metadata::now();
    
    // Verify both IDs are valid hex strings
    assert_valid_protocol_id_hex(&metadata1.request_id);
    assert_valid_protocol_id_hex(&metadata2.request_id);
    
    // Verify IDs are unique (monotonic counter ensures this)
    assert_ne!(metadata1.request_id, metadata2.request_id, "Sequential Metadata::now() calls should generate unique IDs");
    
    assert!(metadata1.timestamp_ms.is_some());
    assert!(metadata2.timestamp_ms.is_some());
}

#[test]
fn test_metadata_with_timestamp_generates_request_id() {
    let timestamp_ms = 1_700_000_000_000;
    
    // Create two metadata instances with same timestamp to verify uniqueness
    let metadata1 = Metadata::with_timestamp(timestamp_ms);
    let metadata2 = Metadata::with_timestamp(timestamp_ms);
    
    assert_eq!(metadata1.timestamp_ms, Some(timestamp_ms));
    assert_eq!(metadata2.timestamp_ms, Some(timestamp_ms));
    
    // Verify both IDs are valid hex strings
    assert_valid_protocol_id_hex(&metadata1.request_id);
    assert_valid_protocol_id_hex(&metadata2.request_id);
    
    // Verify IDs are unique even with same timestamp (monotonic counter ensures this)
    assert_ne!(metadata1.request_id, metadata2.request_id, "Sequential calls should generate unique IDs even with same timestamp");
}

#[test]
fn test_metadata_new_accepts_protocol_id_string() {
    let timestamp_ms = 1_700_000_000_123;
    // Use a fixed, known 32-character hex string to make the intent explicit
    let request_id_str = "00112233445566778899aabbccddeeff".to_string();
    let metadata = Metadata::new(timestamp_ms, request_id_str.clone());
    assert_eq!(metadata.timestamp_ms, Some(timestamp_ms));
    // Verify the provided hex string is actually used
    assert_eq!(metadata.request_id.to_hex(), request_id_str, "Metadata::new should preserve valid request_id");
}

#[test]
fn test_metadata_current_timestamp_ms_non_zero() {
    let now = Metadata::current_timestamp_ms();
    assert!(now > 0);
}

#[test]
fn test_metadata_timestamp_to_string() {
    let metadata = Metadata {
        request_id: ProtocolId::default(),
        timestamp_ms: Some(1234),
        ..Default::default()
    };
    assert_eq!(metadata.timestamp_to_string(), "1+234ms".to_string());
}

#[test]
fn test_metadata_to_key_uses_request_id() {
    let metadata = Metadata {
        request_id: ProtocolId::default(),
        ..Default::default()
    };
    assert_eq!(metadata.to_key(), metadata.request_id.to_string());
}
