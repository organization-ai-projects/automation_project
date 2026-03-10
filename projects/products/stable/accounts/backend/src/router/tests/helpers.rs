//! projects/products/stable/accounts/backend/src/router/tests/helpers.rs
use common_json::{Json, JsonMap};
use protocol::{Command, CommandType, EventType, Metadata, Payload};

use crate::router::helpers::{err_event, get_user_id, parse_permissions};

fn command_with_payload(payload: Json) -> Command {
    Command {
        metadata: Metadata::now(),
        command_type: CommandType::Query,
        action: Some("accounts.get".to_string()),
        payload: Some(Payload {
            payload_type: Some("accounts/get".to_string()),
            payload: Some(payload),
        }),
    }
}

#[test]
fn get_user_id_parses_valid_protocol_id() {
    let mut map = JsonMap::new();
    map.insert(
        "user_id".to_string(),
        Json::String("00000000000000000000000000000000".to_string()),
    );
    let command = command_with_payload(Json::Object(map));

    let user_id = get_user_id(&command).expect("user_id should parse");
    assert_eq!(user_id.to_string(), "00000000000000000000000000000000");
}

#[test]
fn get_user_id_rejects_missing_field() {
    let command = command_with_payload(Json::Object(JsonMap::new()));

    let error = get_user_id(&command).expect_err("missing user_id must fail");
    assert_eq!(error, "Missing user_id");
}

#[test]
fn parse_permissions_rejects_unknown_permission() {
    let values = vec!["read".to_string(), "not_a_permission".to_string()];
    let error = parse_permissions(&values).expect_err("invalid permission must fail");

    assert!(error.contains("Invalid permission"));
    assert!(error.contains("not_a_permission"));
}

#[test]
fn err_event_sets_error_type_and_status_payload() {
    let metadata = Metadata::now();
    let event = err_event(&metadata, 409, "conflict");

    assert_eq!(event.event_type, EventType::Error);
    assert_eq!(event.name, "AccountsError");
    assert_eq!(event.message.as_deref(), Some("conflict"));
    assert!(event.data.contains("conflict"));
}
