// projects/libraries/ai/src/solve_error_kind.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolveErrorKind {
    NotAvailable,
    RuntimeError,
    ValidationError,
}
