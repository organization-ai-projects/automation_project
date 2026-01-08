use backend::router::handle_command;
use protocol::{Command, Metadata, Payload};
use serde_json::json;

#[test]
fn test_handle_preview_command() {
    let cmd = Command {
        metadata: Metadata::now(),
        command_type: protocol::CommandType::Preview,
        action: Some("git_autopilot.preview".to_string()),
        payload: Some(Payload {
            payload_type: Some("git_autopilot/preview/v1".to_string()),
            payload: Some(json!({
                "request_id": "test-request-id-123",
                "details": "Test details"
            })),
        }),
    };

    let response = handle_command(cmd);

    assert_eq!(response.status.code, 200);
    assert_eq!(response.status.description, "Success");
    assert!(
        response.payload.is_some(),
        "La réponse devrait contenir un payload"
    );
}

#[test]
fn test_handle_apply_command() {
    let cmd = Command {
        metadata: Metadata::now(),
        command_type: protocol::CommandType::Apply,
        action: Some("git_autopilot.apply".to_string()),
        payload: Some(Payload {
            payload_type: Some("git_autopilot/apply/v1".to_string()),
            payload: Some(json!({
                "request_id": "test-request-id-456",
                "changes": "Test changes"
            })),
        }),
    };

    let response = handle_command(cmd);

    assert_eq!(response.status.code, 200);
    assert_eq!(response.status.description, "Success");
    assert!(
        response.payload.is_some(),
        "La réponse devrait contenir un payload"
    );
}

#[test]
fn test_handle_unsupported_command() {
    let cmd = Command {
        metadata: Metadata::default(),
        command_type: protocol::CommandType::Custom,
        action: Some("unsupported.command".to_string()),
        payload: None,
    };

    let response = handle_command(cmd);

    assert_eq!(response.status.code, 404);
    assert_eq!(response.status.description, "Not Found");
    assert_eq!(response.message.as_deref(), Some("Commande non supportée"));
}
