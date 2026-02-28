mod execution_result;
#[allow(clippy::module_inception)]
mod executor;
mod executor_error;

pub use execution_result::ExecutionResult;
pub use executor::Executor;
pub use executor_error::ExecutorError;
