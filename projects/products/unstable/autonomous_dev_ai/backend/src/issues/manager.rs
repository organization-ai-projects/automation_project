//! projects/products/unstable/autonomous_dev_ai/backend/src/issues/manager.rs
use std::{
    env,
    process::{self, Command},
    thread,
    time::{Duration, Instant},
};

use crate::{
    audit_event::AuditEvent,
    audit_logger::AuditLogger,
    error::{AgentError, AgentResult},
    ids::{IssueNumber, ParentRef},
    memory_graph::MemoryGraph,
    ops::RunReplay,
    pr_flow::IssueComplianceStatus,
};

use common_json::Json;

pub(crate) fn evaluate_issue_compliance(title: &str, body: &str) -> IssueComplianceStatus {
    if body.trim().is_empty() {
        return IssueComplianceStatus::Unknown;
    }

    if let Some(reason) = validate_required_issue_fields(body) {
        return IssueComplianceStatus::NonCompliant { reason };
    }

    let require_typed_title = env::var("AUTONOMOUS_REQUIRE_TYPED_ISSUE_TITLE")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if require_typed_title && !looks_like_typed_issue_title(title) {
        return IssueComplianceStatus::NonCompliant {
            reason: "title must match type(scope): summary".to_string(),
        };
    }

    IssueComplianceStatus::Compliant
}

pub(crate) fn validate_required_issue_fields(body: &str) -> Option<String> {
    let required_fields =
        env::var("AUTONOMOUS_REQUIRED_ISSUE_FIELDS").unwrap_or_else(|_| "Parent".to_string());

    for raw_field in required_fields.split(',') {
        let field = raw_field.trim();
        if field.is_empty() {
            continue;
        }
        let needle = format!("{field}:");
        let has_field = body
            .lines()
            .any(|line| line.trim_start().starts_with(&needle));
        if !has_field {
            return Some(format!("missing required field: {field}:"));
        }
    }

    let parent_line = body
        .lines()
        .find(|line| line.trim_start().starts_with("Parent:"))
        .map(|line| {
            line.trim_start()
                .trim_start_matches("Parent:")
                .trim()
                .to_string()
        });

    if let Some(parent) = parent_line {
        let Some(parent_ref) = ParentRef::parse(&parent) else {
            return Some("Parent must be '#<number>' or 'none'".to_string());
        };

        if let ParentRef::Issue(parent_issue) = parent_ref {
            if let Ok(issue_number) = env::var("AUTONOMOUS_ISSUE_NUMBER")
                && issue_number == parent_issue.to_string()
            {
                return Some("Parent cannot reference the issue itself".to_string());
            }

            if let Ok(existing_raw) = env::var("AUTONOMOUS_EXISTING_ISSUE_NUMBERS") {
                let exists = existing_raw
                    .split(',')
                    .map(|s| s.trim())
                    .filter_map(ParentRef::parse)
                    .filter_map(|r| match r {
                        ParentRef::Issue(n) => Some(n),
                        ParentRef::None => None,
                    })
                    .any(|n| n == parent_issue);
                if !exists {
                    return Some(format!(
                        "Parent #{} does not exist in known issue set",
                        parent_issue
                    ));
                }
            }
        }
    }

    None
}

fn looks_like_typed_issue_title(title: &str) -> bool {
    let Some(colon_pos) = title.find(':') else {
        return false;
    };
    if colon_pos == 0 || colon_pos + 1 >= title.len() {
        return false;
    }
    let prefix = &title[..colon_pos];
    let has_scope = prefix.contains('(') && prefix.ends_with(')');
    let has_type = prefix
        .chars()
        .take_while(|c| *c != '(')
        .all(|c| c.is_ascii_lowercase() || c == '_' || c == '-');
    has_scope && has_type && !title[colon_pos + 1..].trim().is_empty()
}

pub(crate) fn append_issue_compliance_note(body: &str, status: &IssueComplianceStatus) -> String {
    match status {
        IssueComplianceStatus::Compliant | IssueComplianceStatus::Unknown => body.to_string(),
        IssueComplianceStatus::NonCompliant { reason } => format!(
            "{body}\n\n---\nIssue compliance: non-compliant\nReason: {reason}\nRemediation: fix required issue fields (e.g., Parent: #<id> or Parent: none), then update PR keyword line."
        ),
    }
}

fn load_issue_context(
    issue_number: Option<IssueNumber>,
    goal: &str,
    memory: &mut MemoryGraph,
    run_replay: &mut RunReplay,
    audit: &AuditLogger,
    iteration: usize,
) -> AgentResult<(String, String, String)> {
    let fallback_body = memory
        .metadata
        .get("issue_body")
        .cloned()
        .or_else(|| env::var("AUTONOMOUS_ISSUE_BODY").ok())
        .unwrap_or_default();
    let fallback_title = memory
        .metadata
        .get("issue_title")
        .cloned()
        .or_else(|| env::var("AUTONOMOUS_ISSUE_TITLE").ok())
        .unwrap_or_else(|| goal.to_string());

    let fetch_issue_context_from_gh_enabled = env::var("AUTONOMOUS_FETCH_ISSUE_CONTEXT_FROM_GH")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let fetch_issue_context_required = env::var("AUTONOMOUS_FETCH_ISSUE_CONTEXT_REQUIRED")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if !fetch_issue_context_from_gh_enabled {
        return Ok((fallback_title, fallback_body, "env_or_goal".to_string()));
    }

    let Some(issue_number) = issue_number else {
        if fetch_issue_context_required {
            return Err(AgentError::State(
                "AUTONOMOUS_FETCH_ISSUE_CONTEXT_REQUIRED=true mais aucun numéro d'issue fourni"
                    .to_string(),
            ));
        }
        run_replay.record(
            "issue.context.fetch.skip",
            "gh_enabled_but_missing_issue_number".to_string(),
        );
        return Ok((
            fallback_title,
            fallback_body,
            "fallback_no_issue_number".to_string(),
        ));
    };

    match fetch_issue_context_from_gh(issue_number, Duration::from_secs(30), audit) {
        Ok((title, body)) => Ok((title, body, "gh_issue_view".to_string())),
        Err(error) => {
            if fetch_issue_context_required {
                return Err(error);
            }
            memory.add_failure(
                iteration,
                "Issue context retrieval failed".to_string(),
                error.to_string(),
                Some("Provide AUTONOMOUS_ISSUE_TITLE/AUTONOMOUS_ISSUE_BODY fallback".to_string()),
            );
            run_replay.record("issue.context.fetch.failed", error.to_string());
            Ok((
                fallback_title,
                fallback_body,
                "fallback_fetch_failed".to_string(),
            ))
        }
    }
}

fn fetch_issue_context_from_gh(
    issue_number: IssueNumber,
    tool_timeout: Duration,
    audit: &AuditLogger,
) -> AgentResult<(String, String)> {
    let mut command = Command::new("gh");
    command
        .arg("issue")
        .arg("view")
        .arg(issue_number.to_string())
        .arg("--json")
        .arg("title,body");

    if let Ok(repo) = env::var("AUTONOMOUS_REPO")
        && !repo.trim().is_empty()
    {
        command.arg("--repo").arg(&repo);
    }

    let output = run_command_with_timeout(command, tool_timeout, "gh issue view context")?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    audit.log(AuditEvent::ToolExecution {
        tool: "fetch_issue_context".to_string(),
        args: vec![stdout.clone()],
        success: true,
        timestamp: Instant::now().elapsed().as_secs(),
    })?;

    if !output.status.success() {
        let details = if stderr.is_empty() {
            stdout.clone()
        } else {
            stderr.clone()
        };
        return Err(AgentError::Tool(format!(
            "gh issue view failed (status={}): {}",
            output.status, details
        )));
    }

    let ref root: Json = common_json::from_str(&stdout).map_err(|e| {
        AgentError::State(format!(
            "Failed to parse gh issue context payload as JSON: {}",
            e
        ))
    })?;

    let title = if let Json::Object(map) = root {
        map.get("title")
            .and_then(|v| {
                if let Json::String(s) = v {
                    Some(s.trim().to_string())
                } else {
                    None
                }
            })
            .filter(|v| !v.is_empty())
            .ok_or_else(|| {
                AgentError::State("gh issue context missing non-empty title".to_string())
            })?
    } else {
        return Err(AgentError::State("Invalid JSON structure".to_string()));
    };

    let body = if let Json::Object(map) = root {
        map.get("body")
            .and_then(|v| {
                if let Json::String(s) = v {
                    Some(s.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    } else {
        return Err(AgentError::State("Invalid JSON structure".to_string()));
    };

    Ok((title, body))
}

fn run_command_with_timeout(
    mut command: Command,
    timeout: Duration,
    label: &str,
) -> AgentResult<process::Output> {
    let mut child = command
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .map_err(|e| AgentError::Tool(format!("failed to spawn '{label}': {e}")))?;

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(value)) => {
                println!("Valeur reçue : {:?}", value);
                return child.wait_with_output().map_err(|e| {
                    AgentError::Tool(format!("wait_with_output for '{label}' failed: {e}"))
                });
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait_with_output();
                    return Err(AgentError::Tool(format!(
                        "'{label}' timed out after {}s",
                        timeout.as_secs()
                    )));
                }
                thread::sleep(Duration::from_millis(25));
            }
            Err(e) => {
                return Err(AgentError::Tool(format!(
                    "try_wait for '{label}' failed: {e}"
                )));
            }
        }
    }
}
