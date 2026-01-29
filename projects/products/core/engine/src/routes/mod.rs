// projects/products/core/engine/src/routes/mod.rs

// Module declarations
pub mod accounts;
pub mod auth;
pub mod helpers;
pub mod orchestration;
pub mod projects;
pub mod setup;

// Re-exports from helpers
pub use helpers::http_error;

// Re-exports from auth
pub use auth::{login, normalize_user_id, parse_user_id};

// Re-exports from setup
pub use setup::{health, setup_admin, setup_status};

// Re-exports from projects
pub use projects::list_projects;

// Re-exports from accounts
pub use accounts::{
    create_account, get_account, list_accounts, reset_password, update_account, update_status,
};

// Re-exports from orchestration
pub use orchestration::build_routes;
