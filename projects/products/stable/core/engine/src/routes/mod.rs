// projects/products/stable/core/engine/src/routes/mod.rs

// Module declarations
pub(crate) mod accounts;
pub(crate) mod auth;
pub(crate) mod helpers;
pub(crate) mod http_forwarder;
pub(crate) mod orchestration;
pub(crate) mod projects;
pub(crate) mod setup;

// Re-exports from helpers
pub(crate) use helpers::http_error;

// Re-exports from auth
pub(crate) use auth::login;

// Re-exports from setup
pub(crate) use setup::{health, setup_admin, setup_status};

// Re-exports from projects
pub(crate) use projects::list_projects;

// Re-exports from accounts
pub(crate) use accounts::{
    create_account, get_account, list_accounts, reset_password, update_account, update_status,
};

// Re-exports from orchestration
pub(crate) use orchestration::build_routes;
