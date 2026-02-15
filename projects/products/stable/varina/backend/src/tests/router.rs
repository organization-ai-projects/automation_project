use common_json::{object, to_value};
use protocol::{Command, CommandType, Metadata, Payload, RunRequest};
use serde::Serialize;

use crate::router::{ACTION_GIT_AUTOPILOT_PREVIEW, handle_command};
use crate::validation_error::{E_REPO_PATH_INVALID_FORMAT, E_REPO_PATH_NOT_WHITELISTED};

fn payload_with_type<T: Serialize>(payload_type: &str, payload_value: &T) -> Payload {
    Payload {
        payload_type: Some(payload_type.to_string()),
        payload: Some(to_value(payload_value).expect("valid json payload")),
    }
}

#[track_caller]
fn assert_error_message_contains(response: &protocol::CommandResponse, expected: &str) {
    let err = response.error.as_ref().expect("error should be present");
    assert!(err.message.contains(expected));
}

#[test]
fn handle_command_returns_400_when_action_is_missing() {
    let response = handle_command(Command {
        metadata: Metadata::default(),
        command_type: CommandType::StartJob,
        action: None,
        payload: None,
    });

    assert_eq!(response.status.code, 400);
    assert_error_message_contains(&response, "action is missing");
}

#[test]
fn handle_command_returns_400_when_action_is_blank() {
    let response = handle_command(Command {
        metadata: Metadata::default(),
        command_type: CommandType::StartJob,
        action: Some("   ".to_string()),
        payload: None,
    });

    assert_eq!(response.status.code, 400);
    assert_error_message_contains(&response, "action is missing");
}

#[test]
fn handle_command_returns_404_on_unsupported_action() {
    let response = handle_command(Command {
        metadata: Metadata::default(),
        command_type: CommandType::StartJob,
        action: Some("unsupported.action".to_string()),
        payload: None,
    });

    assert_eq!(response.status.code, 404);
    assert_error_message_contains(&response, "Unsupported command");
}

#[test]
fn handle_command_returns_400_when_preview_payload_is_missing() {
    let response = handle_command(Command {
        metadata: Metadata::default(),
        command_type: CommandType::StartJob,
        action: Some(ACTION_GIT_AUTOPILOT_PREVIEW.to_string()),
        payload: None,
    });

    assert_eq!(response.status.code, 400);
    assert_error_message_contains(&response, "Payload is missing");
}

#[test]
fn handle_command_returns_415_when_payload_type_is_invalid() {
    let response = handle_command(Command {
        metadata: Metadata::default(),
        command_type: CommandType::StartJob,
        action: Some(ACTION_GIT_AUTOPILOT_PREVIEW.to_string()),
        payload: Some(payload_with_type("invalid/type", &object())),
    });

    assert_eq!(response.status.code, 415);
    assert_error_message_contains(&response, "Invalid payload_type");
}

#[test]
fn handle_command_run_rejects_empty_repo_path() {
    let run_req = RunRequest {
        request_id: Default::default(),
        repo_path: Some(String::new()),
    };
    let response = handle_command(Command {
        metadata: Metadata::default(),
        command_type: CommandType::StartJob,
        action: Some("git_autopilot.run".to_string()),
        payload: Some(payload_with_type("git_autopilot/run/v1", &run_req)),
    });

    assert_eq!(response.status.code, 400);
    let err = response.error.as_ref().expect("error should be present");
    assert_eq!(err.code, E_REPO_PATH_INVALID_FORMAT);
    assert!(err.message.contains("cannot be empty"));
}

#[test]
fn handle_command_run_rejects_non_whitelisted_repo_path() {
    let run_req = RunRequest {
        request_id: Default::default(),
        repo_path: Some("/etc/config".to_string()),
    };
    let response = handle_command(Command {
        metadata: Metadata::default(),
        command_type: CommandType::StartJob,
        action: Some("git_autopilot.run".to_string()),
        payload: Some(payload_with_type("git_autopilot/run/v1", &run_req)),
    });

    assert_eq!(response.status.code, 400);
    let err = response.error.as_ref().expect("error should be present");
    assert_eq!(err.code, E_REPO_PATH_NOT_WHITELISTED);
    assert!(err.message.contains("not in the whitelist"));
}
