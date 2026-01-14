use backend::router::{ACTION_GIT_AUTOPILOT_APPLY, ACTION_GIT_AUTOPILOT_PREVIEW, handle_command};
use protocol::json::Json;
use protocol::{Command, CommandType, Metadata, Payload, pjson};

const PREVIEW_PAYLOAD_TYPE_V1: &str = "git_autopilot/preview/v1";
const APPLY_PAYLOAD_TYPE_V1: &str = "git_autopilot/apply/v1";

fn cmd(command_type: CommandType, action: Option<&str>, payload: Option<Payload>) -> Command {
    Command {
        metadata: Metadata::now(),
        command_type,
        action: action.map(|s| s.to_string()),
        payload,
    }
}

fn payload(payload_type: Option<&str>, inner: Option<Json>) -> Payload {
    Payload {
        payload_type: payload_type.map(|s| s.to_string()),
        payload: inner,
    }
}

#[test]
fn router_rejects_missing_action() {
    let res = handle_command(cmd(CommandType::Preview, None, None));

    assert_eq!(res.status.code, 400);
    assert_eq!(res.status.description, "Bad Request");
    assert!(res.error.is_some(), "error doit être Some");
    assert!(
        res.message
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .contains("action"),
        "message devrait mentionner l'action"
    );
}

#[test]
fn router_rejects_unsupported_action() {
    let res = handle_command(cmd(CommandType::Preview, Some("nope/does_not_exist"), None));

    assert_eq!(res.status.code, 404);
    assert_eq!(res.status.description, "Not Found");
    assert_eq!(res.message.as_deref(), Some("Commande non supportée"));
    assert!(res.error.is_some(), "error doit être Some");
}

#[test]
fn router_rejects_missing_payload() {
    let res = handle_command(cmd(
        CommandType::Preview,
        Some(ACTION_GIT_AUTOPILOT_PREVIEW),
        None,
    ));

    assert_eq!(res.status.code, 400);
    assert_eq!(res.status.description, "Bad Request");
    assert!(res.payload.is_none(), "payload doit être None en erreur");
    assert!(res.error.is_some(), "error doit être Some");
    assert!(
        res.message
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .contains("payload"),
        "message devrait mentionner le payload"
    );
}

#[test]
fn router_rejects_invalid_payload_type_preview() {
    let invalid_json = pjson!({});
    let res = handle_command(cmd(
        CommandType::Preview,
        Some(ACTION_GIT_AUTOPILOT_PREVIEW),
        Some(payload(Some("wrong/type"), Some(invalid_json))),
    ));

    assert_eq!(res.status.code, 415);
    assert_eq!(res.status.description, "Unsupported Media Type");
    assert!(res.payload.is_none());
    assert!(res.error.is_some());
    assert!(
        res.message
            .as_deref()
            .unwrap_or("")
            .contains("Invalid payload_type"),
        "message devrait expliquer payload_type invalide"
    );
}

#[test]
fn router_rejects_invalid_payload_type_apply() {
    let invalid_json = pjson!({});
    let res = handle_command(cmd(
        CommandType::Apply,
        Some(ACTION_GIT_AUTOPILOT_APPLY),
        Some(payload(Some("wrong/type"), Some(invalid_json))),
    ));

    assert_eq!(res.status.code, 415);
    assert_eq!(res.status.description, "Unsupported Media Type");
    assert!(res.payload.is_none());
    assert!(res.error.is_some());
}

#[test]
fn router_rejects_missing_inner_payload() {
    let res = handle_command(cmd(
        CommandType::Preview,
        Some(ACTION_GIT_AUTOPILOT_PREVIEW),
        Some(payload(Some(PREVIEW_PAYLOAD_TYPE_V1), None)),
    ));

    assert_eq!(res.status.code, 400);
    assert_eq!(res.status.description, "Bad Request");
    assert!(res.payload.is_none());
    assert!(res.error.is_some());
    assert!(
        res.message
            .as_deref()
            .unwrap_or("")
            .to_lowercase()
            .contains("inner payload"),
        "message devrait mentionner inner payload"
    );
}

#[test]
fn router_rejects_invalid_json_shape_for_preview() {
    let invalid_json = pjson!(null);
    let res = handle_command(cmd(
        CommandType::Preview,
        Some(ACTION_GIT_AUTOPILOT_PREVIEW),
        Some(payload(Some(PREVIEW_PAYLOAD_TYPE_V1), Some(invalid_json))),
    ));

    assert_eq!(res.status.code, 400);
    assert_eq!(res.status.description, "Bad Request");
    assert!(res.payload.is_none());
    assert!(res.error.is_some());
    assert!(
        res.message
            .as_deref()
            .unwrap_or("")
            .contains("Invalid JSON payload"),
        "message devrait mentionner JSON invalide"
    );
}

#[test]
fn router_rejects_invalid_json_shape_for_apply() {
    let invalid_json = pjson!(null);
    let res = handle_command(cmd(
        CommandType::Apply,
        Some(ACTION_GIT_AUTOPILOT_APPLY),
        Some(payload(Some(APPLY_PAYLOAD_TYPE_V1), Some(invalid_json))),
    ));

    assert_eq!(res.status.code, 400);
    assert_eq!(res.status.description, "Bad Request");
    assert!(res.payload.is_none());
    assert!(res.error.is_some());
    assert!(
        res.message
            .as_deref()
            .unwrap_or("")
            .contains("Invalid JSON payload"),
        "message devrait mentionner JSON invalide"
    );
}
