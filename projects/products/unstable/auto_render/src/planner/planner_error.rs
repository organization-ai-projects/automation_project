use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlannerError {
    #[error("Budget exceeded")]
    BudgetExceeded,
    #[error("World query failed")]
    WorldQueryFailed,
    #[error("Constraint unsatisfiable")]
    ConstraintUnsatisfiable,
    #[error("Internal error: {0}")]
    InternalError(String),
}
