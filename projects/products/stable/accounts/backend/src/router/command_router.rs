// projects/products/stable/accounts/backend/src/router/command_router.rs
use protocol::{Command, Event};
use security::TokenService;

use crate::{
    router::{
        err_event, handle_create_user, handle_get_user, handle_list_users, handle_login,
        handle_reset_password, handle_setup_admin, handle_setup_status, handle_update_status,
        handle_update_user,
    },
    store::AccountManager,
};

const ACTION_SETUP_STATUS: &str = "accounts.setup_status";
const ACTION_SETUP_ADMIN: &str = "accounts.setup_admin";
const ACTION_LOGIN: &str = "accounts.login";
const ACTION_LIST_USERS: &str = "accounts.list";
const ACTION_GET_USER: &str = "accounts.get";
const ACTION_CREATE_USER: &str = "accounts.create";
const ACTION_UPDATE_USER: &str = "accounts.update";
const ACTION_UPDATE_STATUS: &str = "accounts.update_status";
const ACTION_RESET_PASSWORD: &str = "accounts.reset_password";

pub const PAYLOAD_SETUP_STATUS: &str = "accounts/setup_status";
pub const PAYLOAD_SETUP_ADMIN: &str = "accounts/setup_admin";
pub const PAYLOAD_LOGIN: &str = "accounts/login";
pub const PAYLOAD_ACCOUNTS_LIST: &str = "accounts/list";
pub const PAYLOAD_ACCOUNT: &str = "accounts/account";
pub const PAYLOAD_OK: &str = "accounts/ok";

pub async fn handle_command(
    cmd: Command,
    manager: &AccountManager,
    token_service: &TokenService,
) -> Event {
    let meta = cmd.metadata.clone();
    let action = match cmd.action.as_deref().map(str::trim) {
        Some(a) if !a.is_empty() => a,
        _ => return err_event(&meta, 400, "Command action is missing"),
    };

    match action {
        ACTION_SETUP_STATUS => handle_setup_status(&meta, manager).await,
        ACTION_SETUP_ADMIN => handle_setup_admin(&meta, &cmd, manager).await,
        ACTION_LOGIN => handle_login(&meta, &cmd, manager, token_service).await,
        ACTION_LIST_USERS => handle_list_users(&meta, manager).await,
        ACTION_GET_USER => handle_get_user(&meta, &cmd, manager).await,
        ACTION_CREATE_USER => handle_create_user(&meta, &cmd, manager).await,
        ACTION_UPDATE_USER => handle_update_user(&meta, &cmd, manager).await,
        ACTION_UPDATE_STATUS => handle_update_status(&meta, &cmd, manager).await,
        ACTION_RESET_PASSWORD => handle_reset_password(&meta, &cmd, manager).await,
        _ => err_event(&meta, 404, "Unsupported action"),
    }
}
