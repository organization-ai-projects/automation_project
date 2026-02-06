// projects/products/unstable/auto_manager_ai/src/lib.rs
// Test-only library interface

pub mod domain;
pub mod adapters;
pub mod ai;

use std::path::{Path, PathBuf};
use std::fs;
use common_json::to_string_pretty;

pub use domain::{Action, ActionPlan, ActionStatus, ActionTarget, Policy, PolicyDecision, PolicyDecisionType, RiskLevel, RunReport};
pub use adapters::{RepoAdapter, GhAdapter, CiAdapter};
pub use ai::{Planner, PlanningContext};

/// Configuration for the automation manager
#[derive(Debug, Clone)]
pub struct Config {
    pub repo_path: PathBuf,
    pub output_dir: PathBuf,
    pub policy: Policy,
}

impl Config {
    /// Create a new configuration
    pub fn new(repo_path: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            repo_path,
            output_dir,
            policy: Policy::default(),
        }
    }
}

/// Generate an action plan based on repository and GitHub context
pub fn generate_action_plan(config: &Config) -> Result<ActionPlan, String> {
    // Create adapters
    let repo_adapter = RepoAdapter::new(config.repo_path.clone());
    let gh_adapter = GhAdapter::new();
    let ci_adapter = CiAdapter::new();

    // Gather context
    let repo_ctx = repo_adapter.get_context()?;
    let gh_ctx = gh_adapter.get_context()?;
    let ci_ctx = ci_adapter.get_context()?;

    // Create planning context
    let planning_ctx = PlanningContext {
        repo: repo_ctx,
        gh: gh_ctx,
        ci: ci_ctx,
    };

    // Generate plan
    Ok(Planner::generate_plan(&planning_ctx))
}

/// Evaluate an action plan against policy
pub fn evaluate_plan(plan: &ActionPlan, policy: &Policy) -> Vec<PolicyDecision> {
    plan.actions
        .iter()
        .map(|action| policy.evaluate(action))
        .collect()
}

/// Write outputs to the output directory
pub fn write_outputs(
    plan: &ActionPlan,
    report: &RunReport,
    out_dir: &Path,
) -> Result<(), String> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(out_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    // Write action plan
    let action_plan_path = out_dir.join("action_plan.json");
    let action_plan_json = to_string_pretty(plan)
        .map_err(|e| format!("Failed to serialize action plan: {:?}", e))?;
    fs::write(&action_plan_path, action_plan_json)
        .map_err(|e| format!("Failed to write action plan: {}", e))?;

    // Write run report
    let run_report_path = out_dir.join("run_report.json");
    let run_report_json = to_string_pretty(report)
        .map_err(|e| format!("Failed to serialize run report: {:?}", e))?;
    fs::write(&run_report_path, run_report_json)
        .map_err(|e| format!("Failed to write run report: {}", e))?;

    Ok(())
}
