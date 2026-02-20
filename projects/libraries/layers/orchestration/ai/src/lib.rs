// projects/libraries/layers/orchestration/ai/src/lib.rs

// Public modules
pub mod ai_body;
pub mod ai_error;
pub mod feedbacks;
pub mod solve_decision;
pub mod solve_error_kind;
pub mod solve_trace;
pub mod solve_winner;
pub mod solver_strategy;
pub mod task;
pub mod task_result;
pub mod task_type;

// Internal modules (not part of public API)
mod ai_orchestrator;
mod dispatcher;
mod solver;
mod training;

// Re-exports for ergonomic usage
pub use ai_body::AiBody;
pub use ai_error::AiError;
pub use solve_decision::SolveDecision;
pub use solve_error_kind::SolveErrorKind;
pub use solve_trace::SolveTrace;
pub use solve_winner::SolveWinner;
pub use solver_strategy::SolverStrategy;
pub use task::Task;
pub use task_result::TaskResult;
pub use task_type::TaskType;
