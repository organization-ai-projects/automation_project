// projects/products/accounts/backend/src/router/setup.rs
use protocol::accounts::{SetupAdminRequest, SetupAdminResponse, SetupStatusResponse};
use protocol::{Command, Event, Metadata};
use security::Role;

use crate::router::helpers::{err_event, map_store_error, ok_payload, payload_as};
use crate::store::account_manager::AccountManager;

use super::command_router::{PAYLOAD_SETUP_ADMIN, PAYLOAD_SETUP_STATUS};

pub async fn handle_setup_status(meta: &Metadata, manager: &AccountManager) -> Event {
    let count = manager.user_count().await;
    let payload = SetupStatusResponse {
        setup_mode: count == 0,
    };
    ok_payload(meta, "SetupStatus", PAYLOAD_SETUP_STATUS, payload)
}

pub async fn handle_setup_admin(meta: &Metadata, cmd: &Command, manager: &AccountManager) -> Event {
    let req: SetupAdminRequest = match payload_as(cmd) {
        Ok(r) => r,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    if req.claim.trim().is_empty() {
        return err_event(meta, 400, "Missing setup claim");
    }

    let role = Role::Admin;
    let create = manager
        .create(
            req.user_id.clone(),
            &req.password,
            role,
            Vec::new(),
            "setup",
        )
        .await;
    match create {
        Ok(_) => ok_payload(
            meta,
            "SetupAdmin",
            PAYLOAD_SETUP_ADMIN,
            SetupAdminResponse { ok: true },
        ),
        Err(err) => map_store_error(meta, err),
    }
}
