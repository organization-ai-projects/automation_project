// projects/products/accounts/backend/src/router/auth.rs
use common::Id128;
use protocol::accounts::{LoginRequest, LoginResponse};
use protocol::{Command, Event, Metadata};
use security::TokenService;

use crate::router::helpers::{err_event, ok_payload, payload_as};
use crate::store::account_manager::AccountManager;

use super::command_router::PAYLOAD_LOGIN;

pub async fn handle_login(
    meta: &Metadata,
    cmd: &Command,
    manager: &AccountManager,
    token_service: &TokenService,
) -> Event {
    let req: LoginRequest = match payload_as(cmd) {
        Ok(r) => r,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    if req.password.trim().is_empty() {
        return err_event(meta, 401, "Invalid credentials");
    }

    let user_id = match Id128::from_hex(&req.user_id) {
        Ok(id) => id,
        Err(_) => return err_event(meta, 400, "Invalid user_id"),
    };

    let role = match manager.authenticate(&req.user_id, &req.password).await {
        Ok(r) => r,
        Err(_) => return err_event(meta, 401, "Invalid credentials"),
    };

    let duration_ms = req.duration_ms.unwrap_or(24 * 60 * 60 * 1000);
    let jwt = match token_service.issue(user_id, role, duration_ms, req.session_id) {
        Ok(t) => t,
        Err(_) => return err_event(meta, 500, "Token issue failed"),
    };

    ok_payload(meta, "Login", PAYLOAD_LOGIN, LoginResponse { jwt })
}
