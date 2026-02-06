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

// ---------- Error codes (stable internal codes) ----------
const E_ACTION_MISSING: i32 = 1000;
const E_ACTION_UNSUPPORTED: i32 = 1004;

const E_PAYLOAD_MISSING: i32 = 1100;
const E_PAYLOAD_TYPE_INVALID: i32 = 1101;
const E_INNER_PAYLOAD_MISSING: i32 = 1102;
const E_PAYLOAD_JSON_INVALID: i32 = 1103;

const E_HANDLER_FAILED: i32 = 1200;
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
            RESPONSE_TYPE_APPLY,
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
    Handler: FnOnce(Req) -> Result<Res, String>,
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
            println!("[error] handle_json: Handler error: {e}");
            return err(
                cmd,
                500,
                "Internal Server Error",
                E_HANDLER_FAILED,
                &format!("Handler error: {e}"),
            );
        }
    };

    ok(cmd, 200, "Success", response_payload_type, &res)
}

fn run_git_autopilot(req: RunRequest) -> Result<String, String> {
    // Validate and resolve the repository path
    let repo_path = match req.repo_path {
        Some(path) => {
            // Validate the provided path using the whitelist
            let validator = RepoPathValidator::default();
            validator.validate(&path).map_err(|e| {
                format!("Repository path validation failed: {}", e.message)
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
        Err(e) => Err(format!("Autopilot error: {}", e)),
    }
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
