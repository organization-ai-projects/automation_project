//! products/unstable/autonomous_dev_ai/backend/src/risks/manager.rs
use std::env;

use crate::{
    lifecycle::{ActionRiskLevel, IterationNumber, MetricsCollector},
    memory_graph::MemoryGraph,
    parsing::parse_risk_level,
    security::PolicyPack,
};

use crate::audit_logger::AuditLogger;
use crate::config::Config;
use crate::ops::RunReplay;

pub(crate) struct RiskManager {
    pub(crate) metrics: MetricsCollector,
    pub(crate) memory: MemoryGraph,
    pub(crate) iteration: IterationNumber,
    pub(crate) config: Config,
    pub(crate) audit: AuditLogger,
    pub(crate) run_replay: RunReplay,
    pub(crate) policy_pack: PolicyPack,
}

impl RiskManager {
    pub(crate) fn adapt_risk_level(&self, base: ActionRiskLevel) -> ActionRiskLevel {
        let recent_avg_failures = self
            .memory
            .metadata
            .get("previous_recent_avg_failures")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let recent_top_failure_kind = self
            .memory
            .metadata
            .get("previous_recent_top_failure_kind")
            .cloned()
            .unwrap_or_default();
        let recent_top_failure_kind_confidence = self
            .memory
            .metadata
            .get("previous_recent_top_failure_kind_confidence")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);

        let should_harden = recent_avg_failures >= 3.0
            || (recent_top_failure_kind.starts_with("timeout:")
                && recent_top_failure_kind_confidence >= 0.6)
            || (recent_top_failure_kind.starts_with("policy:")
                && recent_top_failure_kind_confidence >= 0.6);
        if !should_harden {
            return base;
        }

        match base {
            ActionRiskLevel::Low => ActionRiskLevel::Low,
            ActionRiskLevel::Medium => ActionRiskLevel::High,
            ActionRiskLevel::High => ActionRiskLevel::High,
        }
    }

    pub(crate) fn is_medium_risk_allowed(execution_mode: &str) -> bool {
        let explicit_opt_in = env::var("AUTONOMOUS_ALLOW_MUTATING_TOOLS")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if explicit_opt_in {
            return true;
        }

        !execution_mode.eq_ignore_ascii_case("safe")
    }

    pub(crate) fn action_risk_level(
        tool: &str,
        args: &[String],
        policy_pack: &PolicyPack,
    ) -> ActionRiskLevel {
        if let Some(override_value) = policy_pack.risk_override(tool)
            && let Some(parsed) = parse_risk_level(override_value)
        {
            return parsed;
        }

        if matches!(
            tool,
            "read_file" | "search_code" | "generate_pr_description"
        ) {
            return ActionRiskLevel::Low;
        }
        if tool == "run_tests" && args.first().map(|v| v == "cargo").unwrap_or(false) {
            return ActionRiskLevel::Low;
        }
        if matches!(
            tool,
            "git_commit" | "deploy" | "modify_policy" | "delete_branch"
        ) {
            return ActionRiskLevel::High;
        }
        ActionRiskLevel::Medium
    }

    pub(crate) fn has_valid_high_risk_approval_token() -> bool {
        let provided = env::var("AUTONOMOUS_HIGH_RISK_APPROVAL_TOKEN").ok();
        let expected = env::var("AUTONOMOUS_EXPECTED_APPROVAL_TOKEN").ok();
        match (provided, expected) {
            (Some(p), Some(e)) => !p.is_empty() && p == e,
            _ => false,
        }
    }
}
