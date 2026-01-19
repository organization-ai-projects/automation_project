#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureMode {
    /// Return Err(CommandError::NonZeroExit) if exit code != 0.
    Strict,
    /// Return Ok(Output) even if exit code != 0.
    AllowFailure,
}
