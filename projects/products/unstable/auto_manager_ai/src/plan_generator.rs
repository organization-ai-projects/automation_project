// projects/products/unstable/auto_manager_ai/src/plan_generator.rs

use crate::adapters::{CiAdapter, CiContext, GhAdapter, GhContext, error::AdapterErrorKind};
use crate::ai::{Planner, PlanningContext};
use crate::config::Config;
use crate::domain::ActionPlan;
use crate::engine_gateway::EngineGateway;

/// Generate an action plan based on repository and GitHub context
pub fn generate_action_plan(config: &Config) -> Result<ActionPlan, String> {
    let engine = EngineGateway::new(config.run_mode);
    let gh_adapter = GhAdapter::new();
    let ci_adapter = CiAdapter::new();

    let repo_ctx = engine
        .fetch_repo_context(config.repo_path.clone())
        .map_err(|e| e.render())?;
    let gh_ctx = match gh_adapter.get_context() {
        Ok(ctx) => ctx,
        Err(err) => {
            if matches!(err.kind, AdapterErrorKind::Policy) {
                return Err(err.to_string());
            }
            GhContext {
                available: false,
                status: format!("github adapter degraded: {}", err),
                repo: None,
                default_branch: None,
                open_pr_count: None,
                degraded: true,
                error_code: Some(err.code.to_string()),
            }
        }
    };
    let ci_ctx = match ci_adapter.get_context() {
        Ok(ctx) => ctx,
        Err(err) => {
            if matches!(err.kind, AdapterErrorKind::Policy) {
                return Err(err.to_string());
            }
            CiContext {
                available: false,
                status: format!("ci adapter degraded: {}", err),
                provider: "none".to_string(),
                run_id: None,
                workflow: None,
                job: None,
                degraded: true,
                error_code: Some(err.code.to_string()),
            }
        }
    };

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
    use std::fs;

    use crate::{
        config::{Config, RunMode},
        plan_generator::generate_action_plan,
        tests::test_helpers::create_unique_temp_dir,
    };

    #[test]
    fn test_generate_action_plan() {
        let temp_dir = create_unique_temp_dir("auto_manager_ai_test");
        fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

        let out_dir = temp_dir.join("out");
        let mut config = Config::new(temp_dir.clone(), out_dir);
        config.run_mode = RunMode::DeterministicFallback;

        let result = generate_action_plan(&config);
        assert!(result.is_ok());
        let plan = result.expect("plan");
        assert!(!plan.actions.is_empty());

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_generate_action_plan_includes_missing_input_when_gh_unavailable() {
        let temp_dir = create_unique_temp_dir("auto_manager_ai_test_missing_input");
        fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

        let out_dir = temp_dir.join("out");
        let mut config = Config::new(temp_dir.clone(), out_dir);
        config.run_mode = RunMode::DeterministicFallback;
        let plan = generate_action_plan(&config).expect("plan should generate");

        let has_missing_input = plan
            .actions
            .iter()
            .any(|a| a.missing_inputs.as_ref().is_some_and(|m| !m.is_empty()));
        let has_gh_context_request = plan
            .actions
            .iter()
            .any(|a| a.action_type == "request_gh_context");
        assert_eq!(has_gh_context_request, has_missing_input);
        assert!(!plan.actions.is_empty());

        fs::remove_dir_all(&temp_dir).ok();
    }
}
