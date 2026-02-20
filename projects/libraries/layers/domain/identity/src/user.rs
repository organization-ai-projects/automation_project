// projects/libraries/layers/domain/identity/src/user.rs
use security_core::Role;

use crate::UserId;

/// Represents a stored user with hashed password and role
#[derive(Clone, Debug)]
pub struct User {
    pub user_id: UserId,
    pub password_hash: String,
    pub role: Role,
}
