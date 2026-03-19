//! projects/products/stable/accounts/backend/src/router/mod.rs
mod accounts;
mod auth;
mod command_router;
mod helpers;
mod setup;

pub(crate) use accounts::{
    handle_create_user, handle_get_user, handle_list_users, handle_reset_password,
    handle_update_status, handle_update_user,
};
pub(crate) use auth::handle_login;
pub(crate) use command_router::handle_command;
pub(crate) use helpers::{err_event, get_user_id, map_store_error, ok_payload, ok_payload_json};
pub(crate) use setup::{handle_setup_admin, handle_setup_status};

#[cfg(test)]
mod tests;
