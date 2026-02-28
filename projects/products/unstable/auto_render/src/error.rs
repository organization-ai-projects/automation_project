use thiserror::Error;
use crate::policy::PolicyError;
use crate::planner::PlannerError;
use crate::executor::ExecutorError;
use crate::plan::MigrationError;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Policy error: {0}")]
    Policy(#[from] PolicyError),
    #[error("Planner error: {0}")]
    Planner(#[from] PlannerError),
    #[error("Executor error: {0}")]
    Executor(#[from] ExecutorError),
    #[error("Migration error: {0}")]
    Migration(#[from] MigrationError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
}
