// projects/products/stable/accounts/backend/src/router/accounts.rs
use std::str::FromStr;

use protocol::accounts::{
    AccountStatus, AccountsListResponse, CreateAccountRequest, ResetPasswordRequest,
    UpdateAccountRequest, UpdateStatusRequest,
};
use protocol::{Command, Event, Metadata};
use security::Role;

use crate::router::helpers::{
    err_event, get_user_id, map_store_error, ok_payload, ok_payload_json, parse_permissions,
    payload_as,
};
use crate::store::account_manager::AccountManager;
use crate::store::account_store_error::AccountStoreError;

use super::command_router::{PAYLOAD_ACCOUNT, PAYLOAD_ACCOUNTS_LIST, PAYLOAD_OK};

pub async fn handle_list_users(meta: &Metadata, manager: &AccountManager) -> Event {
    let users = manager.list().await.into_iter().collect::<Vec<_>>();
    ok_payload(
        meta,
        "AccountsList",
        PAYLOAD_ACCOUNTS_LIST,
        AccountsListResponse { users },
    )
}

pub async fn handle_get_user(meta: &Metadata, cmd: &Command, manager: &AccountManager) -> Event {
    let user_id = match get_user_id(cmd) {
        Ok(v) => v,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    match manager.get(&user_id).await {
        Ok(user) => ok_payload(meta, "Account", PAYLOAD_ACCOUNT, user),
        Err(err) => map_store_error(meta, err),
    }
}

pub async fn handle_create_user(meta: &Metadata, cmd: &Command, manager: &AccountManager) -> Event {
    let req: CreateAccountRequest = match payload_as(cmd) {
        Ok(r) => r,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    let role = match Role::from_str(&req.role) {
        Ok(r) => r,
        Err(_) => return map_store_error(meta, AccountStoreError::InvalidRole),
    };

    let permissions = match parse_permissions(&req.permissions) {
        Ok(perms) => perms,
        Err(_) => return map_store_error(meta, AccountStoreError::InvalidPermission),
    };

    match manager
        .create(req.user_id, &req.password, role, permissions, "system")
        .await
    {
        Ok(_) => ok_payload(meta, "AccountCreated", PAYLOAD_OK, ok_payload_json()),
        Err(err) => map_store_error(meta, err),
    }
}

pub async fn handle_update_user(meta: &Metadata, cmd: &Command, manager: &AccountManager) -> Event {
    let user_id = match get_user_id(cmd) {
        Ok(v) => v,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    let req: UpdateAccountRequest = match payload_as(cmd) {
        Ok(r) => r,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    let role = match req.role {
        Some(r) => match Role::from_str(&r) {
            Ok(parsed) => Some(parsed),
            Err(_) => return map_store_error(meta, AccountStoreError::InvalidRole),
        },
        None => None,
    };

    let permissions = match req.permissions {
        Some(p) => match parse_permissions(&p) {
            Ok(perms) => Some(perms),
            Err(_) => return map_store_error(meta, AccountStoreError::InvalidPermission),
        },
        None => None,
    };

    match manager
        .update_role_permissions(&user_id, role, permissions, "system")
        .await
    {
        Ok(_) => ok_payload(meta, "AccountUpdated", PAYLOAD_OK, ok_payload_json()),
        Err(err) => map_store_error(meta, err),
    }
}

pub async fn handle_update_status(
    meta: &Metadata,
    cmd: &Command,
    manager: &AccountManager,
) -> Event {
    let user_id = match get_user_id(cmd) {
        Ok(v) => v,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    let req: UpdateStatusRequest = match payload_as(cmd) {
        Ok(r) => r,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    let status = match AccountStatus::from_str(&req.status) {
        Ok(s) => s,
        Err(_) => return map_store_error(meta, AccountStoreError::InvalidStatus),
    };

    match manager.update_status(&user_id, status, "system").await {
        Ok(_) => ok_payload(meta, "StatusUpdated", PAYLOAD_OK, ok_payload_json()),
        Err(err) => map_store_error(meta, err),
    }
}

pub async fn handle_reset_password(
    meta: &Metadata,
    cmd: &Command,
    manager: &AccountManager,
) -> Event {
    let user_id = match get_user_id(cmd) {
        Ok(v) => v,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    let req: ResetPasswordRequest = match payload_as(cmd) {
        Ok(r) => r,
        Err(msg) => return err_event(meta, 400, &msg),
    };

    match manager
        .reset_password(&user_id, &req.password, "system")
        .await
    {
        Ok(_) => ok_payload(meta, "PasswordReset", PAYLOAD_OK, ok_payload_json()),
        Err(err) => map_store_error(meta, err),
    }
}
