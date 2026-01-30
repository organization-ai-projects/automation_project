// projects/libraries/security/src/lib.rs
pub mod claims;
pub mod password;
pub mod password_error;
pub mod permission_error;
pub mod permissions;
pub mod role;
pub mod token;
pub mod token_error;
pub mod token_service;

pub use claims::Claims;
pub use password::{hash_password, verify_password};
pub use password_error::PasswordError;
pub use permission_error::PermissionError;
pub use permissions::{
    Permission, check_all_permissions, check_permission, check_token_all_permissions,
    check_token_permission, filter_allowed_permissions, has_all_permissions, has_any_permission,
    has_permission, missing_permissions,
};
pub use role::Role;
pub use token::Token;
pub use token_error::TokenError;
pub use token_service::TokenService;

#[cfg(test)]
mod tests;
