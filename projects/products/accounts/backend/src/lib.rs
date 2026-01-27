// projects/products/accounts/backend/src/lib.rs
pub mod router;
pub mod store;

pub use router::{AccountsBackendError, handle_command};
pub use store::{AccountManager, AccountStatus, AccountStoreError, AccountSummary};
