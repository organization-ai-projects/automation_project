// projects/products/unstable/autonomy_orchestrator_ai/src/domain/mod.rs

mod binary_invocation_spec;
mod ci_gate_status;
mod delivery_options;
mod gate_decision;
mod gate_inputs;
mod orchestrator_checkpoint;
mod policy_gate_status;
mod review_gate_status;
mod run_report;
mod stage;
mod stage_execution_record;
mod stage_execution_status;
mod stage_transition;
mod terminal_state;

pub use binary_invocation_spec::BinaryInvocationSpec;
pub use ci_gate_status::CiGateStatus;
pub use delivery_options::DeliveryOptions;
pub use gate_decision::GateDecision;
pub use gate_inputs::GateInputs;
pub use orchestrator_checkpoint::OrchestratorCheckpoint;
pub use policy_gate_status::PolicyGateStatus;
pub use review_gate_status::ReviewGateStatus;
pub use run_report::RunReport;
pub use stage::Stage;
pub use stage_execution_record::StageExecutionRecord;
pub use stage_execution_status::StageExecutionStatus;
pub use stage_transition::StageTransition;
pub use terminal_state::TerminalState;
