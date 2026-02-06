// projects/products/accounts/backend/src/router/mod.rs
pub mod accounts;
pub mod auth;
pub mod command_router;
pub mod helpers;
pub mod setup;

pub use command_router::handle_command;
