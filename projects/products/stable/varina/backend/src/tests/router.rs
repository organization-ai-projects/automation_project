use common_json::{object, to_value};
use protocol::{Command, CommandType, Metadata, Payload, RunRequest};

use crate::router::{ACTION_GIT_AUTOPILOT_PREVIEW, handle_command};
use crate::validation_error::{E_REPO_PATH_INVALID_FORMAT, E_REPO_PATH_NOT_WHITELISTED};

#[test]
fn handle_command_returns_400_when_action_is_missing() {
    let response = handle_command(Command {
        metadata: Metadata::default(),
        command_type: CommandType::StartJob,
        action: None,
        payload: None,
    });

    assert_eq!(response.status.code, 400);
    let err = response.error.expect("error should be present");
    assert!(err.message.contains("action is missing"));
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
    let err = response.error.expect("error should be present");
    assert!(err.message.contains("action is missing"));
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
    let err = response.error.expect("error should be present");
    assert!(err.message.contains("Unsupported command"));
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
    let err = response.error.expect("error should be present");
    assert!(err.message.contains("Payload is missing"));
}

#[test]
fn handle_command_returns_415_when_payload_type_is_invalid() {
    let response = handle_command(Command {
        metadata: Metadata::default(),
        command_type: CommandType::StartJob,
        action: Some(ACTION_GIT_AUTOPILOT_PREVIEW.to_string()),
        payload: Some(Payload {
            payload_type: Some("invalid/type".to_string()),
            payload: Some(object()),
        }),
    });

    assert_eq!(response.status.code, 415);
    let err = response.error.expect("error should be present");
    assert!(err.message.contains("Invalid payload_type"));
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
        payload: Some(Payload {
            payload_type: Some("git_autopilot/run/v1".to_string()),
            payload: Some(to_value(&run_req).expect("valid json")),
        }),
    });

    assert_eq!(response.status.code, 400);
    let err = response.error.expect("error should be present");
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
        payload: Some(Payload {
            payload_type: Some("git_autopilot/run/v1".to_string()),
            payload: Some(to_value(&run_req).expect("valid json")),
        }),
    });

    assert_eq!(response.status.code, 400);
    let err = response.error.expect("error should be present");
    assert_eq!(err.code, E_REPO_PATH_NOT_WHITELISTED);
    assert!(err.message.contains("not in the whitelist"));
}
