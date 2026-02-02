// projects/libraries/protocol/src/metadatas/tests/metadata.rs
use crate::{Metadata, ProtocolId, ValidationError};

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
    let metadata = Metadata::now();
    assert_ne!(metadata.request_id, ProtocolId::default());
    assert!(metadata.timestamp_ms.is_some());
}

#[test]
fn test_metadata_with_timestamp_generates_request_id() {
    let timestamp_ms = 1_700_000_000_000;
    let metadata = Metadata::with_timestamp(timestamp_ms);
    assert_eq!(metadata.timestamp_ms, Some(timestamp_ms));
    assert_ne!(metadata.request_id, ProtocolId::default());
}

#[test]
fn test_metadata_new_accepts_protocol_id_string() {
    let timestamp_ms = 1_700_000_000_123;
    let request_id = ProtocolId::default().to_string();
    let metadata = Metadata::new(timestamp_ms, request_id);
    assert_eq!(metadata.timestamp_ms, Some(timestamp_ms));
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
