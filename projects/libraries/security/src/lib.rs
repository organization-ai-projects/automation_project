// lib.rs

pub fn init() {
    println!("Initializing security library...");
}

pub mod auth;
pub mod permissions;
pub mod role;

pub use auth::{validate_token, Token};
pub use permissions::{has_permission, Role};
