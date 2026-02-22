// projects/products/unstable/autonomy_orchestrator_ai/src/domain/mod.rs

mod binary_invocation_spec;
mod run_report;
mod stage;
mod stage_execution_record;
mod stage_execution_status;
mod stage_transition;
mod terminal_state;

pub use binary_invocation_spec::BinaryInvocationSpec;
pub use run_report::RunReport;
pub use stage::Stage;
pub use stage_execution_record::StageExecutionRecord;
pub use stage_execution_status::StageExecutionStatus;
pub use stage_transition::StageTransition;
pub use terminal_state::TerminalState;
