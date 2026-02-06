// projects/products/unstable/auto_manager_ai/src/plan_generator.rs

use crate::adapters::{CiAdapter, GhAdapter, RepoAdapter};
use crate::ai::{Planner, PlanningContext};
use crate::config::Config;
use crate::domain::ActionPlan;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_generate_action_plan() {
        let temp_dir = std::env::temp_dir().join(format!(
            "auto_manager_ai_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

        let out_dir = temp_dir.join("out");
        let config = Config::new(temp_dir.clone(), out_dir);

        let result = generate_action_plan(&config);
        assert!(result.is_ok());

        fs::remove_dir_all(&temp_dir).ok();
    }
}
