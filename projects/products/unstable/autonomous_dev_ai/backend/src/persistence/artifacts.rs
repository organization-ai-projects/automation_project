//! projects/products/unstable/autonomous_dev_ai/backend/src/persistence/artifacts.rs
use std::{collections::HashMap, env, fs};

use crate::{
    lifecycle::{IterationNumber, MaxIterations, MetricsCollector, RunReport},
    memory_graph::MemoryGraph,
    models::AgentConfig,
    neural::NeuralLayer,
    ops::{RunReplay, build_ops_alerts, render_ops_alerts_markdown},
    path_types::CheckpointPath,
    persistence::ensure_parent_dir_exists,
    security::ActorIdentity,
    tools::ToolMetricSnapshot,
};

pub struct Artifacts {
    pub run_replay: RunReplay,
    pub memory: MemoryGraph,
    pub metrics: MetricsCollector,
    pub state: String,
    pub actor: ActorIdentity,
    pub config: AgentConfig,
    pub neural: NeuralLayer,
    pub current_iteration_number: IterationNumber,
    pub max_iterations_limit: MaxIterations,
    pub checkpoint_path: CheckpointPath,
}

impl Artifacts {
    pub(crate) fn persist_run_replay_artifacts(&mut self) {
        let replay_path = env::var("AUTONOMOUS_RUN_REPLAY_PATH")
            .unwrap_or_else(|_| "agent_run_replay.json".to_string());
        if let Err(e) = self.run_replay.persist(&replay_path) {
            tracing::warn!("Failed to persist run replay '{}': {}", replay_path, e);
        } else {
            self.memory
                .metadata
                .insert("run_replay_path".to_string(), replay_path);
        }

        let replay_text_path = env::var("AUTONOMOUS_RUN_REPLAY_TEXT_PATH")
            .unwrap_or_else(|_| "agent_run_replay.txt".to_string());
        if let Err(e) = fs::write(&replay_text_path, self.run_replay.reconstruct()) {
            tracing::warn!(
                "Failed to persist run replay text '{}': {}",
                replay_text_path,
                e
            );
        } else {
            self.memory
                .metadata
                .insert("run_replay_text_path".to_string(), replay_text_path);
        }
    }

    pub(crate) fn persist_run_report_artifact(&mut self) {
        let metrics_snapshot = self.metrics.snapshot();
        let tool_metrics = build_tool_metric_snapshots(&metrics_snapshot);
        let weighted_objective_score: Option<f64> = self
            .memory
            .metadata
            .get("weighted_objective_score")
            .and_then(|v| v.parse::<f64>().ok());
        let review_required = env::var("AUTONOMOUS_REVIEW_REQUIRED")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let create_pr_enabled = env::var("AUTONOMOUS_CREATE_PR")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let pr_number: Option<u64> = env::var("AUTONOMOUS_PR_NUMBER")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .or_else(|| {
                self.memory
                    .metadata
                    .get("created_pr_number")
                    .and_then(|s| s.parse::<u64>().ok())
            });
        let last_failure = self.memory.failures.last();
        let mut failure_kind_counts: HashMap<String, usize> = HashMap::new();
        for failure in &self.memory.failures {
            let kind = classify_failure_entry(failure);
            *failure_kind_counts.entry(kind).or_insert(0) += 1;
        }
        let top_failure_kind = failure_kind_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(kind, count)| format!("{}:{}", kind, count));
        let authz_denials_total = self
            .memory
            .failures
            .iter()
            .filter(|f| {
                f.description == "Authorization denied"
                    || f.description == "Authorization escalation required"
            })
            .count();
        let policy_violations_total = self
            .memory
            .failures
            .iter()
            .filter(|f| {
                f.description.contains("Policy")
                    || f.error.contains("policy")
                    || f.error.contains("Policy")
            })
            .count();
        let dashboard_json_path = env::var("AUTONOMOUS_OPS_DASHBOARD_JSON_PATH")
            .unwrap_or_else(|_| "test_artifacts/agent_ops_dashboard.json".to_string());
        let dashboard_markdown_path = env::var("AUTONOMOUS_OPS_DASHBOARD_MD_PATH")
            .unwrap_or_else(|_| "test_artifacts/agent_ops_dashboard.md".to_string());
        let non_interactive_profile = self.memory.metadata.get("non_interactive_profile").cloned();
        let runtime_requirements_validated = self
            .memory
            .metadata
            .get("runtime_requirements.validated")
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let pr_readiness = self.memory.metadata.get("pr_readiness").cloned();
        let issue_compliance = self.memory.metadata.get("issue_compliance").cloned();
        let pr_ci_status = self.memory.metadata.get("pr_ci_status").cloned();
        let last_review_outcome = self.memory.metadata.get("last_review_outcome").cloned();
        let closure_gates_satisfied = pr_readiness.as_deref() == Some("ready")
            && issue_compliance.as_deref() == Some("compliant")
            && pr_ci_status.as_deref() == Some("success")
            && (!review_required || last_review_outcome.as_deref() == Some("approved"));
        let mut report = RunReport {
            artifact_schema_version: "1".to_string(),
            artifact_producer: "autonomous_dev_ai".to_string(),
            generated_at_secs: RunReport::now_secs(),
            run_id: self.actor.run_id.to_string(),
            non_interactive_profile,
            runtime_requirements_validated,
            actor_id: self.actor.id.to_string(),
            actor_roles: self
                .actor
                .roles
                .iter()
                .map(|r| format!("{:?}", r))
                .collect(),
            final_state: format!("{:?}", self.state),
            execution_mode: self.config.execution_mode.clone(),
            neural_enabled: self.neural.enabled,
            total_iterations: self.current_iteration_number.get(),
            max_iterations: self.max_iterations_limit.get(),
            total_decisions: self.memory.decisions.len(),
            total_failures: self.memory.failures.len(),
            total_objective_evaluations: self.memory.objective_evaluations.len(),
            explored_files_count: self.memory.explored_files.len(),
            last_objective_passed: self.memory.objective_evaluations.last().map(|e| e.passed),
            weighted_objective_score,
            run_replay_path: self.memory.metadata.get("run_replay_path").cloned(),
            run_replay_text_path: self.memory.metadata.get("run_replay_text_path").cloned(),
            last_tool_failure_class: self.memory.metadata.get("last_tool_failure_class").cloned(),
            review_required,
            create_pr_enabled,
            real_pr_created: self
                .memory
                .metadata
                .get("real_pr_created")
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            pr_number,
            pr_number_source: self.memory.metadata.get("pr_number_source").cloned(),
            pr_ci_status,
            pr_readiness,
            issue_compliance,
            closure_gates_satisfied,
            issue_context_source: self.memory.metadata.get("issue_context_source").cloned(),
            pr_description_source: self.memory.metadata.get("pr_description_source").cloned(),
            last_review_outcome,
            last_review_input_source: self
                .memory
                .metadata
                .get("last_review_input_source")
                .cloned(),
            last_failure_description: last_failure.map(|f| f.description.clone()),
            last_failure_error: last_failure.map(|f| f.error.clone()),
            last_failure_recovery_action: last_failure.and_then(|f| f.recovery_action.clone()),
            failure_kind_counts,
            top_failure_kind,
            last_tool_exit_code: self
                .memory
                .metadata
                .get("last_tool_exit_code")
                .and_then(|v| v.parse::<i32>().ok()),
            last_tool_name: self.memory.metadata.get("last_tool_name").cloned(),
            policy_pack_fingerprint: self.memory.metadata.get("policy_pack.fingerprint").cloned(),
            checkpoint_path: Some(self.checkpoint_path.as_str().to_string()),
            state_transitions_total: metrics_snapshot.state_transitions_total,
            tool_executions_total: metrics_snapshot.tool_executions_total,
            tool_executions_failed: metrics_snapshot.tool_executions_failed,
            risk_gate_allows: metrics_snapshot.risk_gate_allows,
            risk_gate_denies: metrics_snapshot.risk_gate_denies,
            risk_gate_high_approvals: metrics_snapshot.risk_gate_high_approvals,
            authz_denials_total,
            policy_violations_total,
            tool_metrics,
            alerts: Vec::new(),
            dashboard_json_path: Some(dashboard_json_path.clone()),
            dashboard_markdown_path: Some(dashboard_markdown_path.clone()),
        };
        report.alerts = build_ops_alerts(&report);

        let report_path = env::var("AUTONOMOUS_RUN_REPORT_PATH")
            .unwrap_or_else(|_| "agent_run_report.json".to_string());
        match common_json::to_string_pretty(&report) {
            Ok(json) => {
                if let Err(e) = fs::write(&report_path, json) {
                    tracing::warn!("Failed to persist run report '{}': {}", report_path, e);
                } else {
                    self.memory
                        .metadata
                        .insert("run_report_path".to_string(), report_path);
                }
            }
            Err(e) => {
                tracing::warn!("Failed to serialize run report: {}", e);
            }
        }

        self.persist_ops_dashboard_artifacts(
            &report,
            &dashboard_json_path,
            &dashboard_markdown_path,
        );
    }

    pub(crate) fn persist_ops_dashboard_artifacts(
        &mut self,
        report: &RunReport,
        dashboard_json_path: &str,
        dashboard_markdown_path: &str,
    ) {
        if let Err(e) = ensure_parent_dir_exists(dashboard_json_path) {
            tracing::warn!(
                "Failed to create ops dashboard JSON parent directory for '{}': {}",
                dashboard_json_path,
                e
            );
        }
        if let Err(e) = ensure_parent_dir_exists(dashboard_markdown_path) {
            tracing::warn!(
                "Failed to create ops dashboard Markdown parent directory for '{}': {}",
                dashboard_markdown_path,
                e
            );
        }

        match common_json::to_string_pretty(report) {
            Ok(json) => {
                if let Err(e) = fs::write(dashboard_json_path, json) {
                    tracing::warn!(
                        "Failed to persist ops dashboard JSON '{}': {}",
                        dashboard_json_path,
                        e
                    );
                } else {
                    self.memory.metadata.insert(
                        "ops_dashboard_json_path".to_string(),
                        dashboard_json_path.to_string(),
                    );
                }
            }
            Err(e) => tracing::warn!("Failed to serialize ops dashboard JSON: {}", e),
        }

        let md = format!(
            "# Autonomous Ops Dashboard\n\n\
                - Run ID: `{}`\n\
                - Final State: `{}`\n\
                - Iterations: `{}` / `{}`\n\
                - Total Failures: `{}`\n\
                - Policy Violations: `{}`\n\
                - Authz Denials: `{}`\n\
                - Risk Gate Denies: `{}`\n\
                - Top Failure Kind: `{}`\n\n\
                ## Alerts\n{}\n\n\
                ## Tool Metrics\n{}\n",
            report.run_id,
            report.final_state,
            report.total_iterations,
            report.max_iterations,
            report.total_failures,
            report.policy_violations_total,
            report.authz_denials_total,
            report.risk_gate_denies,
            report.top_failure_kind.as_deref().unwrap_or("none"),
            render_ops_alerts_markdown(&report.alerts),
            ToolMetricSnapshot::render_markdown(&report.tool_metrics)
        );

        if let Err(e) = fs::write(dashboard_markdown_path, md) {
            tracing::warn!(
                "Failed to persist ops dashboard Markdown '{}': {}",
                dashboard_markdown_path,
                e
            );
        } else {
            self.memory.metadata.insert(
                "ops_dashboard_markdown_path".to_string(),
                dashboard_markdown_path.to_string(),
            );
        }
    }
}
