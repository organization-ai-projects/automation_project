// projects/libraries/identity/src/lib.rs
pub mod identity_error;
pub mod user_id;
pub mod user_store;

pub use identity_error::IdentityError;
pub use user_id::UserId;
pub use user_store::{User, UserStore};
