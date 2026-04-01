#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum SpreadsheetError {
    #[error("cycle detected in cell dependency graph")]
    CycleDetected,
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("evaluation error: {0}")]
    EvalError(String),
    #[error("unknown cell: {0}")]
    UnknownCell(String),
    #[error("type error: {0}")]
    TypeError(String),
    #[error("division by zero")]
    DivisionByZero,
    #[error("serialization error: {0}")]
    SerializationError(String),
}
