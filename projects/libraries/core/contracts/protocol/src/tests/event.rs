// projects/libraries/protocol/src/tests/event.rs
use super::test_helpers::assert_valid_protocol_id_hex;
use crate::event::{MAX_EVENT_DATA_SIZE, MAX_EVENT_NAME_LENGTH};
use crate::{
    Event, EventType, EventVariant, LogLevel, Metadata, Payload, ProtocolId, ValidationError,
};
use common::custom_uuid::Id128;
use common_json::Json;

fn base_metadata() -> Metadata {
    Metadata {
        request_id: ProtocolId::default(),
        ..Default::default()
    }
}

fn build_event_with_metadata(metadata: Metadata, name: String, data: String) -> Event {
    Event::with_metadata(name, EventType::Custom, data, metadata)
}

#[test]
fn test_event_new_sets_metadata() {
    // Create two events to verify uniqueness
    let event1 = Event::new("name1".to_string(), EventType::Info, "data".to_string());
    let event2 = Event::new("name2".to_string(), EventType::Info, "data".to_string());

    // Verify both IDs are valid hex strings
    assert_valid_protocol_id_hex(&event1.metadata.request_id);
    assert_valid_protocol_id_hex(&event2.metadata.request_id);

    // Verify IDs are unique (not constant/zero)
    assert_ne!(
        event1.metadata.request_id, event2.metadata.request_id,
        "Different events should have unique request_ids"
    );

    assert!(event1.metadata.timestamp_ms.is_some());
    assert!(event2.metadata.timestamp_ms.is_some());
}

#[test]
fn test_event_with_variant_sets_metadata() {
    // Create two events to verify uniqueness
    let event1 = Event::with_variant(
        "name1".to_string(),
        EventType::Info,
        "data".to_string(),
        EventVariant::Default,
    );
    let event2 = Event::with_variant(
        "name2".to_string(),
        EventType::Info,
        "data".to_string(),
        EventVariant::Default,
    );

    // Verify both IDs are valid hex strings
    assert_valid_protocol_id_hex(&event1.metadata.request_id);
    assert_valid_protocol_id_hex(&event2.metadata.request_id);

    // Verify IDs are unique (not constant/zero)
    assert_ne!(
        event1.metadata.request_id, event2.metadata.request_id,
        "Different events should have unique request_ids"
    );

    assert!(event1.metadata.timestamp_ms.is_some());
    assert!(event2.metadata.timestamp_ms.is_some());
}

#[test]
fn test_event_validate_empty_name() {
    let event = build_event_with_metadata(base_metadata(), "".to_string(), "data".to_string());
    let result = event.validate();
    assert!(result.is_err(), "Expected validation error for empty name");
    assert!(matches!(result.unwrap_err(), ValidationError::EmptyName));
}

#[test]
fn test_event_validate_invalid_name_format() {
    let event =
        build_event_with_metadata(base_metadata(), "bad name!".to_string(), "data".to_string());
    let result = event.validate();
    assert!(
        result.is_err(),
        "Expected validation error for invalid name"
    );
    assert!(matches!(
        result.unwrap_err(),
        ValidationError::InvalidNameFormat(_)
    ));
}

#[test]
fn test_event_validate_name_too_long() {
    let name = "a".repeat(MAX_EVENT_NAME_LENGTH + 1);
    let event = build_event_with_metadata(base_metadata(), name, "data".to_string());
    let result = event.validate();
    assert!(result.is_err(), "Expected validation error for name length");
    assert!(matches!(
        result.unwrap_err(),
        ValidationError::NameTooLong { .. }
    ));
}

#[test]
fn test_event_validate_empty_payload() {
    let event =
        build_event_with_metadata(base_metadata(), "valid_name".to_string(), "   ".to_string());
    let result = event.validate();
    assert!(
        result.is_err(),
        "Expected validation error for empty payload"
    );
    assert!(matches!(result.unwrap_err(), ValidationError::EmptyPayload));
}

#[test]
fn test_event_validate_payload_too_large() {
    let data = "a".repeat(MAX_EVENT_DATA_SIZE + 1);
    let event = build_event_with_metadata(base_metadata(), "valid_name".to_string(), data);
    let result = event.validate();
    assert!(
        result.is_err(),
        "Expected validation error for payload size"
    );
    assert!(matches!(
        result.unwrap_err(),
        ValidationError::PayloadTooLarge { .. }
    ));
}

#[test]
fn test_event_validate_allowed_characters() {
    let event = build_event_with_metadata(
        base_metadata(),
        "valid_name-1.2".to_string(),
        "data".to_string(),
    );
    let result = event.validate();
    assert!(
        result.is_ok(),
        "Expected valid event name, got error: {:?}",
        result.err()
    );
}

#[test]
fn test_event_with_metadata_sets_fields() {
    let metadata = base_metadata();
    let event = Event::with_metadata(
        "name".to_string(),
        EventType::Info,
        "data".to_string(),
        metadata.clone(),
    );

    assert_eq!(event.name, "name");
    assert_eq!(event.event_type, EventType::Info);
    assert_eq!(event.data, "data");
    assert_eq!(event.metadata, metadata);
    assert!(event.payload.is_none());
    assert!(matches!(event.variant, EventVariant::Default));
}

#[test]
fn test_event_with_payload_sets_payload_and_data() {
    let metadata = base_metadata();
    let payload = Payload {
        payload_type: Some("json".to_string()),
        payload: Some(Json::String("value".to_string())),
    };

    let event = Event::with_payload("name".to_string(), EventType::Payload, metadata, payload);

    assert_eq!(event.name, "name");
    assert_eq!(event.event_type, EventType::Payload);
    assert!(event.data.contains("value"));
    assert!(event.payload.is_some());
}

#[test]
fn test_event_update_and_extract_payload() {
    let mut event =
        build_event_with_metadata(base_metadata(), "name".to_string(), "data".to_string());
    assert!(event.extract_payload().is_none());

    let payload = Payload {
        payload_type: Some("json".to_string()),
        payload: Some(Json::String("value".to_string())),
    };
    event.update_payload(payload.clone());

    let extracted = event.extract_payload().expect("payload not set");
    assert_eq!(extracted.payload_type, payload.payload_type);
}

#[test]
fn test_event_validate_variant_error() {
    let event = Event {
        name: "valid_name".to_string(),
        event_type: EventType::Error,
        data: "data".to_string(),
        metadata: base_metadata(),
        payload: None,
        level: Some(LogLevel::Error),
        message: None,
        pct: None,
        variant: EventVariant::Error {
            id: Id128::new(0, Some(0), Some(0)),
            message: "".to_string(),
        },
    };

    let result = event.validate();
    assert!(result.is_err(), "Expected validation error for variant");
    assert!(matches!(
        result.unwrap_err(),
        ValidationError::InvalidVariant(_)
    ));
}
