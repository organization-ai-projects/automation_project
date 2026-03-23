//! projects/products/unstable/autonomous_dev_ai/src/pr_flow/pr_orchestrator.rs
use super::{CiStatus, IssueComplianceStatus, MergeReadiness, PrMetadata, ReviewFeedbackIngester};
use crate::issues::{append_issue_compliance_note, evaluate_issue_compliance};
use crate::lifecycle::ActionRiskLevel;
use crate::pr_flow::extract_pr_number_from_text;
use crate::security::{AuthzDecision, SecurityAuditRecord};
use crate::timeout::Timeout;
use crate::{
    audit_logger::AuditLogger,
    error::{AgentError, AgentResult},
    ids::PrNumber,
    memory_graph::MemoryGraph,
    state::AgentState,
};
use std::env;
use std::process::Command;

pub struct PrOrchestrator<'a> {
    pub metadata: PrMetadata,
    pub review_ingester: ReviewFeedbackIngester,
    pub memory: &'a MemoryGraph,
    pub tool_timeout: &'a Timeout,
    pub audit: &'a AuditLogger,
}

impl<'a> PrOrchestrator<'a> {
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        max_review_iterations: usize,
        memory: &'a MemoryGraph,
        tool_timeout: &'a Timeout,
        audit: &'a AuditLogger,
    ) -> Self {
        Self {
            metadata: PrMetadata::new(title, body),
            review_ingester: ReviewFeedbackIngester::new(max_review_iterations),
            memory,
            tool_timeout,
            audit,
        }
    }

    pub fn check_timeout(&self) -> AgentResult<()> {
        Ok(())
    }

    pub fn build_default_pr_body(&self, goal: &str) -> String {
        format!("Default PR body for goal: {}", goal)
    }

    pub fn try_generate_enhanced_pr_description(
        &self,
        goal: &str,
        default_body: &str,
    ) -> Option<String> {
        Some(format!("Enhanced description for goal: {}", goal))
    }

    pub fn load_issue_context(
        &self,
        issue_number: Option<PrNumber>,
        goal: &str,
    ) -> AgentResult<(String, String, String)> {
        Ok((
            format!("Issue title for goal: {}", goal),
            format!("Issue body for goal: {}", goal),
            "context_source".to_string(),
        ))
    }

    pub fn record_replay(&self, kind: &str, payload: impl Into<String>) {
        tracing::info!("Recording replay: {} - {}", kind, payload.into());
    }

    pub fn set_ci_status(&mut self, status: CiStatus) {
        self.metadata.ci_status = status;
    }

    pub fn set_policy_compliant(&mut self, compliant: bool) {
        self.metadata.policy_compliant = compliant;
    }

    pub fn set_issue_compliance(&mut self, status: IssueComplianceStatus) {
        self.metadata.issue_compliance = status;
    }

    pub fn update_body(&mut self, new_body: String) {
        self.metadata.body = new_body;
    }

    pub(crate) fn enforce_authz_for_action(
        &mut self,
        action: &str,
        args: &[String],
    ) -> AgentResult<()> {
        let decision = self
            .authz
            .check_with_args(&self.artifacts.actor, action, args);
        let security_audit = SecurityAuditRecord::new(&self.artifacts.actor, action, &decision);
        let security_payload = common_json::to_string(&security_audit)
            .unwrap_or_else(|_| format!("authz action={} decision={:?}", action, decision));

        let _ = self
            .audit
            .log_symbolic_decision("security_authz", &security_payload);
        self.artifacts
            .run_replay
            .record("security.authz", security_payload);

        if decision.is_allowed() {
            return Ok(());
        }

        match decision {
            AuthzDecision::Deny { reason } => {
                let error = format!("Authorization denied for action '{}': {}", action, reason);
                self.memory.add_failure(
                    self.iteration,
                    "Authorization denied".to_string(),
                    error.clone(),
                    Some("Adjust policy pack/actor permissions for this action".to_string()),
                );
                self.artifacts.run_replay.record(
                    "security.authz.denied",
                    format!("action={} reason=deny", action),
                );
                Err(AgentError::PolicyViolation(error))
            }
            AuthzDecision::RequiresEscalation { required_role } => {
                if has_valid_escalation_approval(&required_role) {
                    self.artifacts.run_replay.record(
                        "security.authz.escalation_approved",
                        format!("action={} required_role={}", action, required_role),
                    );
                    return Ok(());
                }
                let error = format!("Action '{}' requires escalation: {}", action, required_role);
                self.memory.add_failure(
                    self.iteration,
                    "Authorization escalation required".to_string(),
                    error.clone(),
                    Some("Run with required escalation role".to_string()),
                );
                self.artifacts.run_replay.record(
                    "security.authz.denied",
                    format!("action={} reason=requires_escalation", action),
                );
                Err(AgentError::PolicyViolation(error))
            }
            AuthzDecision::Allow => Ok(()),
        }
    }

    pub(crate) fn enforce_risk_gate(&mut self, tool: &str, args: &[String]) -> AgentResult<()> {
        let base_risk = action_risk_level(tool, args, &self.policy_pack);
        let risk = self.adapt_risk_level(base_risk);
        self.run_replay.record(
            "tool.risk_level",
            format!(
                "tool={} base_risk={:?} effective_risk={:?}",
                tool, base_risk, risk
            ),
        );
        let _ = self.audit.log_symbolic_decision(
            "risk_gate_level",
            &format!(
                "tool={} base_risk={:?} effective_risk={:?}",
                tool, base_risk, risk
            ),
        );

        match risk {
            ActionRiskLevel::Low => {
                self.metrics.record_risk_gate_allow();
                let _ = self
                    .audit
                    .log_symbolic_decision("risk_gate_allow", &"low-risk tool allowed".to_string());
                Ok(())
            }
            ActionRiskLevel::Medium => {
                if is_medium_risk_allowed(&self.config.execution_mode) {
                    self.metrics.record_risk_gate_allow();
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_allow",
                        &format!("tool={tool} risk=medium"),
                    );
                    Ok(())
                } else {
                    self.metrics.record_risk_gate_deny();
                    self.memory.add_failure(
                        self.iteration.get(),
                        "Tool execution denied by risk gate".to_string(),
                        format!(
                            "medium-risk tool '{}' denied in mode '{}'",
                            tool, self.config.execution_mode
                        ),
                        Some(
                            "Set AUTONOMOUS_ALLOW_MUTATING_TOOLS=true for explicit opt-in"
                                .to_string(),
                        ),
                    );
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_deny",
                        &format!(
                            "tool={tool} risk=medium mode={}",
                            self.config.execution_mode
                        ),
                    );
                    Err(AgentError::PolicyViolation(format!(
                        "medium-risk tool '{}' denied by safe mode",
                        tool
                    )))
                }
            }
            ActionRiskLevel::High => {
                if !is_medium_risk_allowed(&self.config.execution_mode) {
                    self.metrics.record_risk_gate_deny();
                    self.memory.add_failure(
                        self.iteration.get(),
                        "Tool execution denied by risk gate".to_string(),
                        format!(
                            "high-risk tool '{}' denied in mode '{}'",
                            tool, self.config.execution_mode
                        ),
                        Some(
                            "Enable mutating tools explicitly before high-risk actions".to_string(),
                        ),
                    );
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_deny",
                        &format!("tool={tool} risk=high mode={}", self.config.execution_mode),
                    );
                    return Err(AgentError::PolicyViolation(format!(
                        "high-risk tool '{}' denied by safe mode",
                        tool
                    )));
                }

                if has_valid_high_risk_approval_token() {
                    self.metrics.record_risk_gate_allow();
                    self.metrics.record_risk_gate_high_approval();
                    self.run_replay.record(
                        "tool.high_risk_approved",
                        format!("tool={} token_check=passed", tool),
                    );
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_allow",
                        &format!("tool={tool} risk=high approval=token"),
                    );
                    Ok(())
                } else {
                    self.metrics.record_risk_gate_deny();
                    self.memory.add_failure(
                            self.iteration.get(),
                            "High-risk approval missing".to_string(),
                            format!("tool '{}' requires explicit approval token", tool),
                            Some(
                                "Set AUTONOMOUS_HIGH_RISK_APPROVAL_TOKEN and AUTONOMOUS_EXPECTED_APPROVAL_TOKEN to matching values".to_string(),
                            ),
                        );
                    let _ = self.audit.log_symbolic_decision(
                        "risk_gate_deny",
                        &format!("tool={tool} risk=high approval=missing_or_invalid"),
                    );
                    Err(AgentError::PolicyViolation(format!(
                        "high-risk tool '{}' requires approval token",
                        tool
                    )))
                }
            }
        }
    }

    pub(crate) fn create_pr(&mut self) -> AgentResult<()> {
        self.check_timeout()?;

        tracing::info!(
            "Iteration {}: Creating pull request",
            self.memory
                .metadata
                .get("current_iteration_number")
                .unwrap_or(&"0".to_string())
        );

        let goal = self
            .memory
            .metadata
            .get("goal")
            .unwrap_or(&"No goal".to_string())
            .clone();

        let default_pr_body = self.build_default_pr_body(&goal);

        let pr_body = self
            .try_generate_enhanced_pr_description(&goal, &default_pr_body)
            .unwrap_or(default_pr_body);
        let require_generated_pr_description =
            env::var("AUTONOMOUS_REQUIRE_GENERATED_PR_DESCRIPTION")
                .ok()
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
        if require_generated_pr_description {
            let source = self
                .memory
                .metadata
                .get("pr_description_source")
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            if source != "generated" {
                return Err(AgentError::State(format!(
                    "AUTONOMOUS_REQUIRE_GENERATED_PR_DESCRIPTION=true but source is '{}'",
                    source
                )));
            }
        }

        let issue_number = extract_issue_number_from_goal(&goal);
        let (issue_title, issue_body, issue_context_source) =
            self.load_issue_context(issue_number, &goal)?;
        self.memory.metadata.insert(
            "issue_context_source".to_string(),
            issue_context_source.clone(),
        );
        self.record_replay("issue.context_source", issue_context_source);
        let issue_compliance = evaluate_issue_compliance(&issue_title, &issue_body);
        let require_issue_compliance = env::var("AUTONOMOUS_REQUIRE_ISSUE_COMPLIANCE")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if require_issue_compliance && !matches!(issue_compliance, IssueComplianceStatus::Compliant)
        {
            return Err(AgentError::State(format!(
                "AUTONOMOUS_REQUIRE_ISSUE_COMPLIANCE=true but issue compliance is {:?}",
                issue_compliance
            )));
        }
        let pr_body = append_issue_compliance_note(&pr_body, &issue_compliance);
        let mut opened_pr = self.metadata.pr_number.is_some();
        let mut real_pr_created = false;
        let mut pr_number_source = if opened_pr {
            self.memory
                .metadata
                .get("pr_number_source")
                .cloned()
                .unwrap_or_else(|| "existing_orchestrator".to_string())
        } else {
            "none".to_string()
        };
        if let Ok(pr_number_raw) = env::var("AUTONOMOUS_PR_NUMBER")
            && let Ok(parsed) = pr_number_raw.parse::<u64>()
            && let Some(prn) = PrNumber::new(parsed)
        {
            self.open(prn);
            opened_pr = true;
            pr_number_source = "env_injected".to_string();
        }
        if let Some(n) = issue_number {
            self.record_replay("issue.reference", format!("issue_number={}", n.get()));
            self.metadata.close_issue(n);
        }
        self.set_ci_status(match self.memory.metadata.get("last_validation_success") {
            Some(v) if v == "true" => CiStatus::Passing,
            Some(_) => CiStatus::Failing,
            None => CiStatus::Unknown,
        });
        let fetch_pr_ci_status = env::var("AUTONOMOUS_FETCH_PR_CI_STATUS_FROM_GH")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let fetch_pr_ci_status_required = env::var("AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let policy_ok = !self
            .memory
            .failures
            .iter()
            .any(|f| f.description.to_ascii_lowercase().contains("policy"));
        self.set_policy_compliant(policy_ok);
        self.set_issue_compliance(issue_compliance.clone());
        let rendered_body = self.metadata.render_body();
        self.update_body(rendered_body.clone());
        let create_pr_enabled = env::var("AUTONOMOUS_CREATE_PR")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let create_pr_required = env::var("AUTONOMOUS_CREATE_PR_REQUIRED")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if create_pr_required && !create_pr_enabled {
            return Err(AgentError::State(
                "AUTONOMOUS_CREATE_PR_REQUIRED=true requires AUTONOMOUS_CREATE_PR=true".to_string(),
            ));
        }
        if create_pr_enabled && !opened_pr {
            let precheck_result = if !self.policy.is_tool_allowed("create_pr") {
                Err(AgentError::PolicyViolation(
                    "Tool 'create_pr' is not allowed by policy".to_string(),
                ))
            } else {
                self.enforce_authz_for_action("create_pr", &[])
                    .and_then(|_| self.enforce_risk_gate("create_pr", &[]))
            };
            if let Err(error) = precheck_result {
                if create_pr_required {
                    return Err(error);
                }
                self.memory.add_failure(
                    self.iteration,
                    "Real PR creation blocked by policy gate".to_string(),
                    error.to_string(),
                    Some(
                        "Adjust policy/authz/risk settings or disable AUTONOMOUS_CREATE_PR"
                            .to_string(),
                    ),
                );
            } else {
                match self.try_create_real_pr(&self.metadata.title, &rendered_body) {
                    Ok(Some(real_pr)) => {
                        self.open(real_pr);
                        opened_pr = true;
                        real_pr_created = true;
                        pr_number_source = "gh_created".to_string();
                    }
                    Ok(None) => {}
                    Err(error) => {
                        if create_pr_required {
                            return Err(error);
                        }
                        self.memory.add_failure(
                        self.iteration,
                        "Real PR creation failed".to_string(),
                        error.to_string(),
                        Some(
                            "Set AUTONOMOUS_CREATE_PR_REQUIRED=true to fail-closed on PR creation"
                                .to_string(),
                        ),
                    );
                    }
                }
            }
        }
        let require_real_pr_creation = env::var("AUTONOMOUS_REQUIRE_REAL_PR_CREATION")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if require_real_pr_creation && !create_pr_enabled {
            return Err(AgentError::State(
                "AUTONOMOUS_REQUIRE_REAL_PR_CREATION=true requires AUTONOMOUS_CREATE_PR=true"
                    .to_string(),
            ));
        }
        if require_real_pr_creation && !real_pr_created {
            return Err(AgentError::State(
                "AUTONOMOUS_REQUIRE_REAL_PR_CREATION=true but no PR was created via GitHub API path"
                    .to_string(),
            ));
        }

        let require_pr_number = env::var("AUTONOMOUS_REQUIRE_PR_NUMBER")
            .ok()
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        if require_pr_number && !opened_pr {
            return Err(AgentError::State(
                "AUTONOMOUS_REQUIRE_PR_NUMBER=true but no valid PR number is available (AUTONOMOUS_PR_NUMBER or real creation)"
                    .to_string(),
            ));
        }
        if fetch_pr_ci_status {
            if let Some(pr_number) = self.metadata.pr_number {
                match self.fetch_pr_ci_status_from_gh(pr_number) {
                    Ok(ci_status) => {
                        self.set_ci_status(ci_status.clone());
                        self.memory
                            .metadata
                            .insert("pr_ci_status".to_string(), format!("{:?}", ci_status));
                    }
                    Err(error) => {
                        if fetch_pr_ci_status_required {
                            return Err(error);
                        }
                        self.memory.add_failure(
                            self.iteration,
                            "PR CI status retrieval failed".to_string(),
                            error.to_string(),
                            Some(
                                "Set AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED=true to fail-closed"
                                    .to_string(),
                            ),
                        );
                    }
                }
            } else if fetch_pr_ci_status_required {
                return Err(AgentError::State(
                    "AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED=true but no PR number is available"
                        .to_string(),
                ));
            }
        }

        let readiness = self.merge_readiness();
        let readiness_ok = readiness.is_ready();
        let readiness_msg = match &readiness {
            MergeReadiness::Ready => "ready".to_string(),
            MergeReadiness::NotReady { reasons } => format!("not_ready: {}", reasons.join(" | ")),
        };
        self.memory
            .metadata
            .insert("pr_readiness".to_string(), readiness_msg.clone());
        self.memory.metadata.insert(
            "pr_ci_status".to_string(),
            format!("{:?}", self.metadata.ci_status),
        );
        self.memory.metadata.insert(
            "real_pr_created".to_string(),
            if real_pr_created { "true" } else { "false" }.to_string(),
        );
        self.memory
            .metadata
            .insert("pr_number_source".to_string(), pr_number_source);
        self.memory.metadata.insert(
            "issue_compliance".to_string(),
            format!("{:?}", issue_compliance),
        );
        self.record_replay("pr.readiness", readiness_msg.clone());
        self.record_replay("issue.compliance", format!("{:?}", issue_compliance));
        self.record_replay("pr.rendered_body", format!("chars={}", rendered_body.len()));
        self.pr_orchestrator = Some(self.clone());

        self.log_symbolic_decision_safe("create_pr", &rendered_body)?;
        self.log_symbolic_decision_safe("merge_readiness", &readiness_msg)?;
        if !readiness_ok {
            self.memory.add_failure(
                self.iteration,
                "PR merge readiness not met".to_string(),
                readiness_msg.clone(),
                Some("continue through review loop for remediation".to_string()),
            );
        }

        if opened_pr {
            tracing::info!("PR is tracked with body ({} chars)", pr_body.len());
        } else if create_pr_enabled {
            tracing::info!(
                "PR creation requested but no PR number available (body {} chars)",
                pr_body.len()
            );
        } else {
            tracing::info!(
                "PR creation not enabled; metadata prepared with body ({} chars)",
                pr_body.len()
            );
        }

        self.transition_to(AgentState::ReviewFeedback)
    }

    pub(crate) fn try_create_real_pr(
        &mut self,
        title: &str,
        body: &str,
    ) -> AgentResult<Option<PrNumber>> {
        let mut command = Command::new("gh");
        command.arg("pr").arg("create").arg("--title").arg(title);
        command.arg("--body").arg(body);
        let mut audit_args = vec![
            "pr".to_string(),
            "create".to_string(),
            "--title".to_string(),
            title.to_string(),
            "--body".to_string(),
            "<omitted>".to_string(),
        ];

        if let Ok(base) = env::var("AUTONOMOUS_PR_BASE")
            && !base.trim().is_empty()
        {
            command.arg("--base").arg(base);
            audit_args.push("--base".to_string());
            audit_args.push("<set>".to_string());
        }
        if let Ok(head) = env::var("AUTONOMOUS_PR_HEAD")
            && !head.trim().is_empty()
        {
            command.arg("--head").arg(head);
            audit_args.push("--head".to_string());
            audit_args.push("<set>".to_string());
        }
        if let Ok(repo) = env::var("AUTONOMOUS_REPO")
            && !repo.trim().is_empty()
        {
            command.arg("--repo").arg(&repo);
            audit_args.push("--repo".to_string());
            audit_args.push(repo);
        }

        let output = run_command_with_timeout(command, self.tool_timeout.duration, "gh pr create")?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let _ = self
            .audit
            .log_tool_execution("create_pr", &audit_args, output.status.success());

        if !output.status.success() {
            let details = if stderr.is_empty() {
                stdout.clone()
            } else {
                stderr.clone()
            };
            let failure_class = classify_tool_failure(&details);
            self.record_replay(
                "tool.failure_class",
                format!("tool=create_pr class={}", failure_class),
            );
            self.memory.metadata.insert(
                "last_tool_failure_class".to_string(),
                format!("create_pr:{}", failure_class),
            );
            return Err(AgentError::Tool(format!(
                "gh pr create failed (status={}): {}",
                output.status, details
            )));
        }

        let pr_number = extract_pr_number_from_text(&stdout)
            .or_else(|| extract_pr_number_from_text(&stderr))
            .and_then(PrNumber::new);
        if let Some(pr) = pr_number {
            self.record_replay("pr.created", format!("number={}", pr));
            self.memory
                .metadata
                .insert("created_pr_number".to_string(), pr.to_string());
            return Ok(Some(pr));
        }

        self.record_replay(
            "pr.created",
            "number_unavailable_from_gh_output".to_string(),
        );
        Ok(None)
    }
}
