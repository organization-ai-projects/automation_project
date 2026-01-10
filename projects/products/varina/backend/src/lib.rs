/// projects/products/varina/backend/src/lib.rs
pub mod app;
pub mod automation;
pub mod autopilot;
pub mod cargo;
pub mod classified_changes;
pub mod classified_changes_ref;
pub mod git_github;
pub mod policy_evaluation;
pub mod pre_checks;
pub mod router;

pub use autopilot::{
    AutopilotError, AutopilotMode, AutopilotPlan, AutopilotPolicy, AutopilotReport,
};
pub use classified_changes::ClassifiedChanges;
pub use classified_changes_ref::ClassifiedChangesRef;
pub use policy_evaluation::{
    classify_changes, display_change_path, has_merge_conflicts, is_blocked, is_relevant,
};
pub use pre_checks::PreChecks;
pub use router::handle_command;
