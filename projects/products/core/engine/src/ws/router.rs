// projects/products/core/engine/src/ws/router.rs
use common_json::{pjson, to_value};
use protocol::{Command, Event, ProjectMetadata};
use security::{Permission, Token};
use tracing::{info, warn};

use crate::{
    EngineState, require_permission, require_project_exists,
    ws::{ws_event_error, ws_event_ok, ws_event_ok_payload},
};

/// Command router "engine-level".
pub(crate) async fn route_command(
    cmd: Command,
    state: &EngineState,
    token: &Token,
    _perms: &[Permission], // to be implemented as soon as possible
) -> Event {
    let meta = cmd.metadata.clone();

    let action = match cmd.action.as_deref().map(str::trim) {
        Some(a) if !a.is_empty() => a,
        _ => return ws_event_error(&meta, 400, 1000, "Command action is missing"),
    };

    match action {
        "engine.ping" => {
            info!("WS cmd: engine.ping (subject_id={})", token.subject_id);
            ws_event_ok(&meta, "Pong")
        }

        "engine.list_projects" => {
            info!(
                "WS cmd: engine.list_projects (subject_id={})",
                token.subject_id
            );

            if let Err(e) = require_permission(token, Permission::Read) {
                return ws_event_error(&meta, 403, 1003, e);
            }

            let reg = state.registry.read().await;
            let list: Vec<ProjectMetadata> = reg.projects.values().cloned().collect();

            let value = to_value(&list).unwrap_or_else(|_| pjson!([]));
            ws_event_ok_payload(&meta, "ProjectsListed", "engine/projects", value)
        }

        // Project-specific actions
        action if action.starts_with("project.") => {
            info!("WS cmd: {} (subject_id={})", action, token.subject_id);

            if let Err(e) = require_project_exists(&cmd, state).await {
                return ws_event_error(&meta, 404, 1001, e);
            }

            let required_perm = if action.contains("read") || action.contains("get") {
                Permission::Read
            } else if action.contains("write") || action.contains("update") {
                Permission::Write
            } else if action.contains("delete") {
                Permission::Delete
            } else {
                Permission::Execute
            };

            if let Err(e) = require_permission(token, required_perm) {
                return ws_event_error(&meta, 403, 1003, e);
            }

            ws_event_error(&meta, 501, 1005, "Project actions not implemented yet")
        }

        other => {
            warn!(
                "WS cmd: unsupported action '{}' (subject_id={})",
                other, token.subject_id
            );
            ws_event_error(&meta, 404, 1004, format!("Unsupported action: {}", other))
        }
    }
}
