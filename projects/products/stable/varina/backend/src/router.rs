// projects/products/varina/backend/src/router.rs
use serde::Serialize;
use serde::de::DeserializeOwned;

use common_json::{from_value, to_string, to_value};
use protocol::{
    Command, CommandResponse, Metadata, ProtocolError, ResponseStatus, apply_request::ApplyRequest,
    payload::Payload, preview_request::PreviewRequest, run_request::RunRequest,
};

use crate::automation::run_git_autopilot_in_repo;
use crate::autopilot::{AutopilotMode, AutopilotPolicy};
use crate::autopilot::{handle_apply_git_autopilot, handle_preview_git_autopilot};
use crate::handler_error::HandlerError;
use crate::repo_path_validator::RepoPathValidator;

// ---------- Routing constants (future proof) ----------
pub const ACTION_GIT_AUTOPILOT_PREVIEW: &str = "git_autopilot/preview";
pub const ACTION_GIT_AUTOPILOT_APPLY: &str = "git_autopilot/apply";
const ACTION_GIT_AUTOPILOT_RUN: &str = "git_autopilot.run";

// Payload type (v2 ready). Activate them when you want strict versioning.
const PAYLOAD_TYPE_PREVIEW_V1: &str = "git_autopilot/preview/v1";
const PAYLOAD_TYPE_APPLY_V1: &str = "git_autopilot/apply/v1";
const PAYLOAD_TYPE_RUN_V1: &str = "git_autopilot/run/v1";

// Response payload types
const RESPONSE_TYPE_PREVIEW: &str = "preview_response";
const RESPONSE_TYPE_APPLY: &str = "apply_response";
const RESPONSE_TYPE_RUN: &str = "run_response";

// ---------- Error codes (stable internal codes) ----------
const E_ACTION_MISSING: i32 = 1000;
const E_ACTION_UNSUPPORTED: i32 = 1004;

const E_PAYLOAD_MISSING: i32 = 1100;
const E_PAYLOAD_TYPE_INVALID: i32 = 1101;
const E_INNER_PAYLOAD_MISSING: i32 = 1102;
const E_PAYLOAD_JSON_INVALID: i32 = 1103;

pub const E_HANDLER_FAILED: i32 = 1200;
#[allow(dead_code)]
const E_AUTOPILOT_FAILED: i32 = 1300;

const E_SERIALIZE_MESSAGE: i32 = 1400;
const E_SERIALIZE_PAYLOAD: i32 = 1401;

pub fn handle_command(cmd: Command) -> CommandResponse {
    let action = match cmd.action.as_deref().map(str::trim) {
        Some(a) if !a.is_empty() => a,
        _ => {
            return err(
                &cmd,
                400,
                "Bad Request",
                E_ACTION_MISSING,
                "Command action is missing",
            );
        }
    };

    match action {
        ACTION_GIT_AUTOPILOT_PREVIEW => handle_json::<PreviewRequest, _, _>(
            &cmd,
            Some(PAYLOAD_TYPE_PREVIEW_V1),
            handle_preview_git_autopilot,
            RESPONSE_TYPE_PREVIEW,
        ),
        ACTION_GIT_AUTOPILOT_APPLY => handle_json::<ApplyRequest, _, _>(
            &cmd,
            Some(PAYLOAD_TYPE_APPLY_V1),
            handle_apply_git_autopilot,
            RESPONSE_TYPE_APPLY,
        ),
        ACTION_GIT_AUTOPILOT_RUN => handle_json::<RunRequest, _, _>(
            &cmd,
            Some(PAYLOAD_TYPE_RUN_V1),
            run_git_autopilot,
            RESPONSE_TYPE_RUN,
        ),
        _ => err(
            &cmd,
            404,
            "Not Found",
            E_ACTION_UNSUPPORTED,
            "Unsupported command",
        ),
    }
}

/// Generic JSON command handler:
/// - validates cmd.payload + optional payload_type
/// - deserializes payload into Req
/// - calls handler
/// - serializes handler result into CommandResponse payload/message
fn handle_json<Req, Handler, Res>(
    cmd: &Command,
    expected_payload_type: Option<&str>,
    handler: Handler,
    response_payload_type: &str,
) -> CommandResponse
where
    Req: DeserializeOwned,
    Res: Serialize + Clone,
    Handler: FnOnce(Req) -> Result<Res, HandlerError>,
{
    let payload = match cmd.payload.as_ref() {
        Some(p) => p,
        None => {
            println!("[error] handle_json: Payload is missing");
            return err(
                cmd,
                400,
                "Bad Request",
                E_PAYLOAD_MISSING,
                "Payload is missing",
            );
        }
    };

    if let Some(expected) = expected_payload_type {
        let got = payload.payload_type.as_deref().unwrap_or("").trim();
        if got != expected {
            println!(
                "[error] handle_json: Invalid payload_type. Expected: '{}', Got: '{}'",
                expected, got
            );
            return err(
                cmd,
                415,
                "Unsupported Media Type",
                E_PAYLOAD_TYPE_INVALID,
                &format!("Invalid payload_type: expected '{expected}', got '{got}'"),
            );
        }
    }

    let inner_ref = match payload.payload.as_ref() {
        Some(v) => v,
        None => {
            println!("[error] handle_json: Inner payload is missing");
            return err(
                cmd,
                400,
                "Bad Request",
                E_INNER_PAYLOAD_MISSING,
                "Inner payload is missing",
            );
        }
    };

    println!("[debug] handle_json: Received payload: {:?}", inner_ref);
    println!(
        "[debug] handle_json: Validating payload type: {:?}",
        payload.payload_type
    );
    println!(
        "[debug] handle_json: Inner payload before deserialization: {:?}",
        inner_ref
    );
    println!("[debug] handle_json: Inspecting inner_ref: {:?}", inner_ref);
    println!("[debug] handle_json: Attempting to deserialize payload into target structure");

    let req: Req = match from_value(inner_ref.clone()) {
        Ok(r) => r,
        Err(e) => {
            println!("[error] handle_json: Invalid JSON payload: {e}");
            return err(
                cmd,
                400,
                "Bad Request",
                E_PAYLOAD_JSON_INVALID,
                &format!("Invalid JSON payload: {e}"),
            );
        }
    };

    let res = match handler(req) {
        Ok(r) => r,
        Err(e) => {
            println!("[error] handle_json: Handler error: {}", e.message);
            return err(
                cmd,
                e.http_code,
                if e.http_code == 400 { "Bad Request" } else { "Internal Server Error" },
                e.error_code,
                &e.message,
            );
        }
    };

    ok(cmd, 200, "Success", response_payload_type, &res)
}

fn run_git_autopilot(req: RunRequest) -> Result<String, HandlerError> {
    // Validate and resolve the repository path
    let repo_path = match req.repo_path {
        Some(path) => {
            // Create validator with whitelist from environment or default
            let validator = create_repo_path_validator();
            validator.validate(&path).map_err(|e| {
                HandlerError::validation_error(e.code, format!("Repository path validation failed: {}", e.message))
            })?
        }
        None => {
            // Fallback to environment variable or current directory (existing behavior)
            crate::automation::resolve_repo_path()
        }
    };

    let mode = AutopilotMode::ApplySafe;
    let policy = AutopilotPolicy::default();

    match run_git_autopilot_in_repo(&repo_path, mode, &policy) {
        Ok(report) => Ok(format!("Success: {:?}", report)),
        Err(e) => Err(HandlerError::internal_error(E_HANDLER_FAILED, format!("Autopilot error: {}", e))),
    }
}

/// Create a RepoPathValidator with whitelist from environment or default.
///
/// Checks for `VARINA_REPO_WHITELIST` environment variable which should contain
/// comma-separated absolute paths (e.g., "/home,/tmp,/workspace").
///
/// Falls back to default whitelist (/home, /tmp, /workspace) if:
/// - Environment variable is not set
/// - Environment variable is empty
/// - All paths in environment variable are empty after trimming
fn create_repo_path_validator() -> RepoPathValidator {
    use std::env;
    use std::path::PathBuf;

    // Check for VARINA_REPO_WHITELIST environment variable
    // Format: comma-separated paths like "/home,/tmp,/workspace"
    if let Ok(whitelist_str) = env::var("VARINA_REPO_WHITELIST") {
        let whitelist: Vec<PathBuf> = whitelist_str
            .split(',')
            .map(|s| PathBuf::from(s.trim()))
            .filter(|p| !p.as_os_str().is_empty())
            .collect();
        
        if !whitelist.is_empty() {
            println!("[info] Using repo path whitelist from VARINA_REPO_WHITELIST: {:?}", whitelist);
            return RepoPathValidator::new(whitelist);
        }
    }

    // Fall back to default whitelist
    RepoPathValidator::default()
}

/// Success response builder (no unwrap, full error mapping)
fn ok<T: Serialize + Clone>(
    cmd: &Command,
    http_code: u16,
    description: &str,
    payload_type: &str,
    res: &T,
) -> CommandResponse {
    let message = match to_string(res) {
        Ok(s) => s,
        Err(e) => {
            return err(
                cmd,
                500,
                "Internal Server Error",
                E_SERIALIZE_MESSAGE,
                &format!("Serialization error (message): {e}"),
            );
        }
    };

    let payload_value = match to_value(&res.clone()) {
        Ok(v) => v,
        Err(e) => {
            return err(
                cmd,
                500,
                "Internal Server Error",
                E_SERIALIZE_PAYLOAD,
                &format!("Serialization error (payload): {e}"),
            );
        }
    };

    CommandResponse {
        metadata: meta(cmd),
        status: ResponseStatus {
            code: http_code,
            description: description.to_string(),
        },
        message: Some(message),
        payload: Some(Payload {
            payload_type: Some(payload_type.to_string()),
            payload: Some(payload_value),
        }),
        error: None,
    }
}

/// Error response builder (single source of truth)
fn err(cmd: &Command, http_code: u16, desc: &str, code: i32, msg: &str) -> CommandResponse {
    CommandResponse {
        metadata: meta(cmd),
        status: ResponseStatus {
            code: http_code,
            description: desc.to_string(),
        },
        message: Some(msg.to_string()),
        payload: None,
        error: Some(ProtocolError {
            code,
            message: msg.to_string(),
        }),
    }
}

/// Metadata echo for correlation/tracing.
/// Strongly recommended: derive Clone on Metadata in protocol.
fn meta(cmd: &Command) -> Metadata {
    cmd.metadata.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_json::to_value;
    use protocol::{CommandType, ProtocolId};
    use crate::validation_error::{E_REPO_PATH_INVALID_FORMAT, E_REPO_PATH_NOT_WHITELISTED};

    fn create_test_metadata() -> Metadata {
        Metadata {
            request_id: ProtocolId::default(),
            job_id: None,
            product_id: None,
            client_id: None,
            timestamp_ms: None,
            schema_version: None,
        }
    }

    #[test]
    fn test_run_git_autopilot_with_invalid_path() {
        let req = RunRequest {
            request_id: ProtocolId::default(),
            repo_path: Some("/etc/../../../etc/passwd".to_string()),
        };

        let result = run_git_autopilot(req);
        assert!(result.is_err(), "Expected validation to fail for path: /etc/../../../etc/passwd");
        let err = result.unwrap_err();
        // The path may be canonicalized which would resolve the traversal
        // So we check for either path traversal or whitelist error
        assert!(
            err.message.contains("Path traversal") || err.message.contains("not in the whitelist"),
            "Expected path traversal or whitelist error, got: {}",
            err.message
        );
    }

    #[test]
    fn test_run_git_autopilot_with_empty_path() {
        let req = RunRequest {
            request_id: ProtocolId::default(),
            repo_path: Some("".to_string()),
        };

        let result = run_git_autopilot(req);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("cannot be empty"));
    }

    #[test]
    fn test_run_git_autopilot_with_non_whitelisted_path() {
        let req = RunRequest {
            request_id: ProtocolId::default(),
            repo_path: Some("/etc/config".to_string()),
        };

        let result = run_git_autopilot(req);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("not in the whitelist"));
    }

    #[test]
    fn test_run_git_autopilot_with_no_path_uses_fallback() {
        let req = RunRequest {
            request_id: ProtocolId::default(),
            repo_path: None,
        };

        // This should not fail validation since it uses the fallback
        // The actual autopilot execution might fail, but not due to validation
        let result = run_git_autopilot(req);
        // We expect either success or an autopilot error (not a validation error)
        if let Err(e) = result {
            // Should not be a validation error
            assert!(!e.message.contains("Path traversal"));
            assert!(!e.message.contains("not in the whitelist"));
            assert!(!e.message.contains("cannot be empty"));
        }
    }

    #[test]
    fn test_handle_command_with_missing_action() {
        let cmd = Command {
            metadata: create_test_metadata(),
            command_type: CommandType::Execute,
            action: None,
            payload: None,
        };

        let response = handle_command(cmd);
        assert_eq!(response.status.code, 400);
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, E_ACTION_MISSING);
    }

    #[test]
    fn test_handle_command_with_empty_action() {
        let cmd = Command {
            metadata: create_test_metadata(),
            command_type: CommandType::Execute,
            action: Some("   ".to_string()),
            payload: None,
        };

        let response = handle_command(cmd);
        assert_eq!(response.status.code, 400);
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, E_ACTION_MISSING);
    }

    #[test]
    fn test_handle_command_with_unsupported_action() {
        let cmd = Command {
            metadata: create_test_metadata(),
            command_type: CommandType::Execute,
            action: Some("unsupported.action".to_string()),
            payload: None,
        };

        let response = handle_command(cmd);
        assert_eq!(response.status.code, 404);
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, E_ACTION_UNSUPPORTED);
    }

    #[test]
    fn test_handle_command_with_missing_payload() {
        let cmd = Command {
            metadata: create_test_metadata(),
            command_type: CommandType::Execute,
            action: Some(ACTION_GIT_AUTOPILOT_PREVIEW.to_string()),
            payload: None,
        };

        let response = handle_command(cmd);
        assert_eq!(response.status.code, 400);
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, E_PAYLOAD_MISSING);
    }

    #[test]
    fn test_handle_command_with_invalid_payload_type() {
        let cmd = Command {
            metadata: create_test_metadata(),
            command_type: CommandType::Execute,
            action: Some(ACTION_GIT_AUTOPILOT_PREVIEW.to_string()),
            payload: Some(Payload {
                payload_type: Some("invalid/type".to_string()),
                payload: Some(to_value(&"{}").unwrap()),
            }),
        };

        let response = handle_command(cmd);
        assert_eq!(response.status.code, 415);
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, E_PAYLOAD_TYPE_INVALID);
    }

    // Router-level validation tests

    #[test]
    fn test_handle_command_run_with_empty_repo_path() {
        let run_req = RunRequest {
            request_id: ProtocolId::default(),
            repo_path: Some("".to_string()),
        };
        
        let cmd = Command {
            metadata: create_test_metadata(),
            command_type: CommandType::Execute,
            action: Some(ACTION_GIT_AUTOPILOT_RUN.to_string()),
            payload: Some(Payload {
                payload_type: Some(PAYLOAD_TYPE_RUN_V1.to_string()),
                payload: Some(to_value(&run_req).unwrap()),
            }),
        };

        let response = handle_command(cmd);
        assert_eq!(response.status.code, 400, "Expected 400 Bad Request for empty path");
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, E_REPO_PATH_INVALID_FORMAT, "Expected E_REPO_PATH_INVALID_FORMAT error code");
        assert!(error.message.contains("cannot be empty"), "Error message should mention empty path");
    }

    #[test]
    fn test_handle_command_run_with_non_whitelisted_path() {
        let run_req = RunRequest {
            request_id: ProtocolId::default(),
            repo_path: Some("/etc/config".to_string()),
        };
        
        let cmd = Command {
            metadata: create_test_metadata(),
            command_type: CommandType::Execute,
            action: Some(ACTION_GIT_AUTOPILOT_RUN.to_string()),
            payload: Some(Payload {
                payload_type: Some(PAYLOAD_TYPE_RUN_V1.to_string()),
                payload: Some(to_value(&run_req).unwrap()),
            }),
        };

        let response = handle_command(cmd);
        assert_eq!(response.status.code, 400, "Expected 400 Bad Request for non-whitelisted path");
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, E_REPO_PATH_NOT_WHITELISTED, "Expected E_REPO_PATH_NOT_WHITELISTED error code");
        assert!(error.message.contains("not in the whitelist"), "Error message should mention whitelist");
    }

    #[test]
    fn test_handle_command_run_with_valid_whitelisted_path() {
        let run_req = RunRequest {
            request_id: ProtocolId::default(),
            repo_path: Some("/tmp/test-repo".to_string()),
        };
        
        let cmd = Command {
            metadata: create_test_metadata(),
            command_type: CommandType::Execute,
            action: Some(ACTION_GIT_AUTOPILOT_RUN.to_string()),
            payload: Some(Payload {
                payload_type: Some(PAYLOAD_TYPE_RUN_V1.to_string()),
                payload: Some(to_value(&run_req).unwrap()),
            }),
        };

        let response = handle_command(cmd);
        // Should not get validation error (might get autopilot error if path doesn't exist)
        if response.status.code == 400 {
            let error = response.error.unwrap();
            assert!(
                error.code != E_REPO_PATH_INVALID_FORMAT && error.code != E_REPO_PATH_NOT_WHITELISTED,
                "Should not get validation error for whitelisted path"
            );
        }
    }
}
