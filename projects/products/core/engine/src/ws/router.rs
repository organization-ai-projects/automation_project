// projects/products/core/engine/src/ws/router.rs
use common_json::{from_value, pjson, to_value};
use protocol::{Command, Event};
use security::{Permission, Token};
use tracing::{info, warn};

use crate::runtime::BackendRegistry;
use crate::{
    EngineState, ProjectMetadata, require_permission, require_project_exists,
    ws::{BackendRegistration, ws_event_error, ws_event_ok, ws_event_ok_payload},
};

/// Command router "engine-level".
pub async fn route_command(
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

        "backend.hello" => {
            info!("WS cmd: backend.hello (subject_id={})", token.subject_id);

            // Require a dedicated permission
            if let Err(e) = require_permission(token, Permission::Execute) {
                return ws_event_error(&meta, 403, 1003, e);
            }

            let payload_value = match cmd.payload.as_ref().and_then(|p| p.payload.as_ref()) {
                Some(v) => v.clone(),
                None => {
                    warn!("Missing cmd.payload.payload for backend.hello");
                    return ws_event_error(&meta, 400, 1002, "Missing payload");
                }
            };

            let registration: BackendRegistration = match from_value(payload_value) {
                Ok(h) => h,
                Err(e) => {
                    warn!("Failed to parse BackendRegistration: {e}");
                    return ws_event_error(&meta, 400, 1000, "Invalid payload");
                }
            };

            info!(
                "Backend hello: product_id={} instance_id={} capabilities={} routes={}",
                registration.product_id,
                registration.instance_id,
                registration.capabilities.len(),
                registration.routes.len()
            );

            // Register in the runtime registry
            let mut backends = state.backend_registry.write().await;
            let backends: &mut BackendRegistry = &mut backends;
            backends.register(
                registration.product_id,
                registration.instance_id,
                registration.capabilities,
                registration.routes,
            );

            ws_event_ok(&meta, "BackendRegistered")
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
