// projects/products/accounts/backend/src/router.rs
use protocol::{Command, Event};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountsBackendError {
    #[error("unhandled command")]
    Unhandled,
}

pub fn handle_command(_cmd: Command) -> Result<Event, AccountsBackendError> {
    Err(AccountsBackendError::Unhandled)
}
