// projects/products/stable/core/engine/src/requires.rs
use protocol::Command;
use security::{Permission, Token};

use crate::EngineState;

pub(crate) async fn require_project_exists(
    cmd: &Command,
    state: &EngineState,
) -> Result<(), String> {
    let key = cmd.metadata.to_key();
    let reg = state.registry.read().await;
    if !reg.projects.contains_key(&key) {
        return Err(format!("Project not found: {}", key));
    }
    Ok(())
}

/// Checks the token's permissions
pub(crate) fn require_permission(token: &Token, perm: Permission) -> Result<(), String> {
    if !token.role.permissions().contains(&perm) {
        return Err(format!("Missing permission: {:?}", perm));
    }
    Ok(())
}
