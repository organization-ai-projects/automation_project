// projects/libraries/identity/src/lib.rs
pub mod identity_error;
pub mod user;
pub mod user_id;
pub mod user_store;

pub use identity_error::IdentityError;
pub use user::User;
pub use user_id::UserId;
pub use user_store::UserStore;

#[cfg(test)]
mod tests;
