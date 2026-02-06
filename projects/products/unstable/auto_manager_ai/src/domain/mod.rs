// projects/products/unstable/auto_manager_ai/src/domain/mod.rs

pub mod risk_level;
pub mod action_target;
pub mod evidence;
pub mod action_status;
pub mod dry_run_step;
pub mod dry_run;
pub mod action;
pub mod action_plan;
pub mod policy_decision_type;
pub mod policy_decision;
pub mod policy;
pub mod run_status;
pub mod run_output;
pub mod run_report;

pub use action::Action;
pub use action_plan::ActionPlan;
pub use action_status::ActionStatus;
pub use action_target::ActionTarget;
pub use dry_run::DryRun;
pub use dry_run_step::DryRunStep;
pub use evidence::Evidence;
pub use risk_level::RiskLevel;
pub use policy::Policy;
pub use policy_decision::PolicyDecision;
pub use policy_decision_type::PolicyDecisionType;
pub use run_report::RunReport;
pub use run_status::RunStatus;
pub use run_output::RunOutput;
