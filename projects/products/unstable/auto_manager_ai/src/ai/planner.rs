// projects/products/unstable/auto_manager_ai/src/ai/planner.rs

use super::planning_context::PlanningContext;
use crate::domain::{Action, ActionPlan, ActionStatus, ActionTarget, Evidence, RiskLevel};

/// AI planner (template-based for V0)
pub struct Planner;

impl Planner {
    /// Generate an action plan based on context
    pub fn generate_plan(ctx: &PlanningContext) -> ActionPlan {
        let summary = format!(
            "Context-aware plan: repo_files={}, engine_mediated={}, gh_available={}, ci_available={}",
            ctx.repo.tracked_files.len(),
            ctx.repo.mediated_by_engine,
            ctx.gh.available,
            ctx.ci.available
        );
        let mut plan = ActionPlan::new(summary);

        let mut evidence = vec![
            Evidence {
                source: "repo_context".to_string(),
                pointer: format!(
                    "tracked_files={} engine_mediated={}",
                    ctx.repo.tracked_files.len(),
                    ctx.repo.mediated_by_engine
                ),
            },
            Evidence {
                source: "gh_context".to_string(),
                pointer: format!(
                    "status={} repo={} default_branch={} open_pr_count={}",
                    ctx.gh.status,
                    ctx.gh.repo.clone().unwrap_or_else(|| "n/a".to_string()),
                    ctx.gh
                        .default_branch
                        .clone()
                        .unwrap_or_else(|| "n/a".to_string()),
                    ctx.gh.open_pr_count.unwrap_or_default()
                ),
            },
            Evidence {
                source: "ci_context".to_string(),
                pointer: format!(
                    "status={} provider={} run_id={} workflow={} job={} degraded={}",
                    ctx.ci.status,
                    ctx.ci.provider,
                    ctx.ci.run_id.clone().unwrap_or_else(|| "n/a".to_string()),
                    ctx.ci.workflow.clone().unwrap_or_else(|| "n/a".to_string()),
                    ctx.ci.job.clone().unwrap_or_else(|| "n/a".to_string()),
                    ctx.ci.degraded
                ),
            },
        ];
        if let Some(code) = &ctx.gh.error_code {
            evidence.push(Evidence {
                source: "gh_context_error".to_string(),
                pointer: code.clone(),
            });
        }
        if let Some(code) = &ctx.ci.error_code {
            evidence.push(Evidence {
                source: "ci_context_error".to_string(),
                pointer: code.clone(),
            });
        }

        plan.add_action(Action {
            id: "action_001".to_string(),
            action_type: "collect_repo_inventory".to_string(),
            status: ActionStatus::Proposed,
            target: ActionTarget::Repo {
                reference: format!("{}", ctx.repo.root.display()),
            },
            justification: "Collect repository inventory using engine-mediated context".to_string(),
            risk_level: RiskLevel::Low,
            required_checks: vec![
                "engine_available".to_string(),
                "policy_allow".to_string(),
                "authz_allow".to_string(),
            ],
            confidence: if ctx.repo.mediated_by_engine {
                0.95
            } else {
                0.75
            },
            evidence,
            depends_on: None,
            missing_inputs: None,
            dry_run: None,
        });

        if !ctx.gh.available || ctx.gh.degraded {
            plan.add_action(Action {
                id: "action_002".to_string(),
                action_type: "request_gh_context".to_string(),
                status: ActionStatus::NeedsInput,
                target: ActionTarget::Repo {
                    reference: format!("{}", ctx.repo.root.display()),
                },
                justification: "GitHub context degraded; requires operator confirmation"
                    .to_string(),
                risk_level: RiskLevel::Low,
                required_checks: vec!["gh_context_required".to_string()],
                confidence: 0.6,
                evidence: vec![Evidence {
                    source: "gh_context".to_string(),
                    pointer: ctx.gh.status.clone(),
                }],
                depends_on: Some(vec!["action_001".to_string()]),
                missing_inputs: Some(vec!["github_context".to_string()]),
                dry_run: None,
            });
        }

        plan
    }
}
