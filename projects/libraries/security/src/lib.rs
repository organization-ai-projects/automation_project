// security/src/lib.rs
pub mod auth;
pub mod auth_error;
pub mod permissions;
pub mod role;
pub mod token;
pub mod token_error;
pub mod token_service;
pub mod claims;

pub use crate::token::Token;
pub use auth_error::AuthError;
pub use permissions::{
    Permission, check_all_permissions, check_permission, check_token_all_permissions,
    check_token_permission, filter_allowed_permissions, has_all_permissions, has_any_permission,
    has_permission, missing_permissions,
};
pub use role::Role;

pub fn init() {
    println!("Initializing security library...");
}
pub use token_error::TokenError;
pub use token_service::TokenService;
pub use claims::Claims;
