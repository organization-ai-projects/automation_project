use common_json::{from_str, to_string_pretty};
use serde::{Deserialize, Serialize};
use std::fs;
use std::mem;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::artifacts::{
    NextActionsArtifact, RepoContextArtifactCompat, ValidationInvocationArtifact, load_next_actions,
};
use crate::domain::{CommandLineSpec, Stage, StageExecutionStatus, TerminalState};
use crate::versioning::VersioningCommands;

const PLANNING_FEEDBACK_SCHEMA_VERSION: u32 = 1;
const MAX_FEEDBACK_ITEMS: usize = 8;
const MAX_FEEDBACK_TEXT_LEN: usize = 240;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoContextArtifact {
    pub repo_root: String,
    pub generated_at_unix_secs: u64,
    pub top_level_entries: Vec<String>,
    pub workspace_members: Vec<String>,
    pub ownership_boundaries: Vec<String>,
    pub hot_paths: Vec<String>,
    pub detected_validation_commands: Vec<ValidationInvocationArtifact>,
    pub planning_feedback: Option<PlanningFeedbackArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningFeedbackArtifact {
    pub schema_version: u32,
    pub source_run_id: Option<String>,
    pub terminal_state: Option<String>,
    pub blocked_reason_codes: Vec<String>,
    pub reviewer_next_steps: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub validation_outcomes: Vec<PlanningValidationOutcome>,
    pub feedback_signature: String,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningValidationOutcome {
    pub command: String,
    pub status: String,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
struct RunReportFeedbackCompat {
    run_id: String,
    terminal_state: Option<TerminalState>,
    blocked_reason_codes: Option<Vec<String>>,
    reviewer_next_steps: Option<Vec<String>>,
    stage_executions: Option<Vec<StageExecutionFeedbackCompat>>,
}

#[derive(Debug, Clone, Deserialize)]
struct StageExecutionFeedbackCompat {
    stage: Stage,
    status: StageExecutionStatus,
    command: String,
    exit_code: Option<i32>,
}

pub fn write_repo_context_artifact(
    repo_root: &Path,
    artifact_path: &Path,
    previous_run_report_path: Option<&Path>,
    next_actions_path: Option<&Path>,
) -> Result<(), String> {
    let top_level_entries = list_top_level_entries(repo_root)?;
    let workspace_members = detect_workspace_members(repo_root);
    let ownership_boundaries = detect_ownership_boundaries(repo_root);
    let hot_paths = detect_hot_paths(repo_root);
    let detected_validation_commands = detect_validation_commands(repo_root);
    let planning_feedback = extract_planning_feedback(previous_run_report_path, next_actions_path);
    let payload = RepoContextArtifact {
        repo_root: repo_root.display().to_string(),
        generated_at_unix_secs: unix_timestamp_secs(),
        top_level_entries,
        workspace_members,
        ownership_boundaries,
        hot_paths,
        detected_validation_commands,
        planning_feedback,
    };

    if let Some(parent) = artifact_path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create planning context parent dir '{}': {}",
                parent.display(),
                e
            )
        })?;
    }

    let json = to_string_pretty(&payload)
        .map_err(|e| format!("Failed to serialize planning repo context artifact: {e:?}"))?;
    fs::write(artifact_path, json).map_err(|e| {
        format!(
            "Failed to write planning repo context artifact '{}': {}",
            artifact_path.display(),
            e
        )
    })
}

pub fn read_detected_validation_commands(
    artifact_path: &Path,
) -> Result<Vec<ValidationInvocationArtifact>, String> {
    let raw = fs::read_to_string(artifact_path).map_err(|e| {
        format!(
            "Failed to read planning context artifact '{}': {}",
            artifact_path.display(),
            e
        )
    })?;
    let parsed: RepoContextArtifactCompat = from_str(&raw).map_err(|e| {
        format!(
            "Failed to parse planning context artifact '{}': {:?}",
            artifact_path.display(),
            e
        )
    })?;
    Ok(match parsed.detected_validation_commands {
        VersioningCommands::Current(commands) => commands,
        VersioningCommands::Legacy(commands) => commands
            .into_iter()
            .filter_map(|command| {
                let tokens = command
                    .split_whitespace()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                let (head, tail) = tokens.split_first()?;
                Some(ValidationInvocationArtifact {
                    command_line: CommandLineSpec {
                        command: head.clone(),
                        args: tail.to_vec(),
                    },
                })
            })
            .collect(),
    })
}

fn list_top_level_entries(repo_root: &Path) -> Result<Vec<String>, String> {
    let mut entries = fs::read_dir(repo_root)
        .map_err(|e| format!("Failed to read repo root '{}': {}", repo_root.display(), e))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect::<Vec<_>>();
    entries.sort_unstable();
    entries.truncate(200);
    Ok(entries)
}

fn detect_validation_commands(repo_root: &Path) -> Vec<ValidationInvocationArtifact> {
    let mut commands = Vec::new();
    if path_exists(repo_root, "Cargo.toml") {
        commands.push(ValidationInvocationArtifact {
            command_line: CommandLineSpec {
                command: "cargo".to_string(),
                args: vec!["fmt", "--all", "--", "--check"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
        });
        commands.push(ValidationInvocationArtifact {
            command_line: CommandLineSpec {
                command: "cargo".to_string(),
                args: vec![
                    "clippy",
                    "--workspace",
                    "--all-targets",
                    "--",
                    "-D",
                    "warnings",
                ]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            },
        });
        commands.push(ValidationInvocationArtifact {
            command_line: CommandLineSpec {
                command: "cargo".to_string(),
                args: vec!["test", "--workspace"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
        });
    }
    if path_exists(repo_root, "package.json") {
        commands.push(ValidationInvocationArtifact {
            command_line: CommandLineSpec {
                command: "npm".to_string(),
                args: vec!["run", "lint"]
                    .into_iter()
                    .map(ToString::to_string)
                    .collect(),
            },
        });
        commands.push(ValidationInvocationArtifact {
            command_line: CommandLineSpec {
                command: "npm".to_string(),
                args: vec!["test"].into_iter().map(ToString::to_string).collect(),
            },
        });
    }
    commands
}

fn detect_workspace_members(repo_root: &Path) -> Vec<String> {
    let cargo_toml = repo_root.join("Cargo.toml");
    let raw = match fs::read_to_string(&cargo_toml) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut members = Vec::new();
    let mut in_workspace = false;
    let mut in_members = false;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_workspace = trimmed == "[workspace]";
            in_members = false;
            continue;
        }
        if !in_workspace {
            continue;
        }
        if trimmed.starts_with("members") && trimmed.contains('[') {
            in_members = true;
        }
        if in_members {
            if let Some(start) = trimmed.find('"')
                && let Some(end) = trimmed[start + 1..].find('"')
            {
                let member = &trimmed[start + 1..start + 1 + end];
                if !member.is_empty() {
                    members.push(member.to_string());
                }
            }
            if trimmed.contains(']') {
                in_members = false;
            }
        }
    }
    members.sort_unstable();
    members
}

fn detect_ownership_boundaries(repo_root: &Path) -> Vec<String> {
    let mut boundaries = Vec::new();
    for candidate in [
        "projects/products/stable",
        "projects/products/unstable",
        "projects/libraries",
        "scripts",
        "documentation",
    ] {
        if path_exists(repo_root, candidate) {
            boundaries.push(candidate.to_string());
        }
    }
    boundaries
}

fn detect_hot_paths(repo_root: &Path) -> Vec<String> {
    let mut hot_paths = Vec::new();
    for candidate in [
        "projects/products/unstable/autonomy_orchestrator_ai/src",
        "projects/products/unstable/autonomous_dev_ai/src",
        "projects/products/unstable/auto_manager_ai/src",
        "projects/products/unstable/autonomy_reviewer_ai/src",
        "scripts/automation",
    ] {
        if path_exists(repo_root, candidate) {
            hot_paths.push(candidate.to_string());
        }
    }
    hot_paths
}

fn path_exists(root: &Path, relative: &str) -> bool {
    let mut full = PathBuf::from(root);
    full.push(relative);
    full.exists()
}

fn extract_planning_feedback(
    previous_run_report_path: Option<&Path>,
    next_actions_path: Option<&Path>,
) -> Option<PlanningFeedbackArtifact> {
    let run_report = read_run_report_feedback(previous_run_report_path);
    let next_actions = read_next_actions_feedback(next_actions_path);
    if run_report.is_none() && next_actions.is_none() {
        return None;
    }

    let source_run_id = run_report
        .as_ref()
        .map(|report| report.run_id.clone())
        .or_else(|| next_actions.as_ref().map(|next| next.run_id.clone()));
    let terminal_state = run_report
        .as_ref()
        .and_then(|report| report.terminal_state)
        .or_else(|| next_actions.as_ref().and_then(|next| next.terminal_state))
        .map(terminal_state_label)
        .map(ToString::to_string);

    let mut blocked_reason_codes = run_report
        .as_ref()
        .and_then(|report| report.blocked_reason_codes.clone())
        .unwrap_or_default();
    if let Some(next) = &next_actions {
        blocked_reason_codes.extend(next.blocked_reason_codes.iter().cloned());
    }

    let mut reviewer_next_steps = run_report
        .as_ref()
        .and_then(|report| report.reviewer_next_steps.clone())
        .unwrap_or_default();
    if let Some(next) = &next_actions {
        reviewer_next_steps.extend(next.reviewer_next_steps.iter().cloned());
    }

    let recommended_actions = next_actions
        .as_ref()
        .map(|next| next.recommended_actions.clone())
        .unwrap_or_default();
    let validation_outcomes = extract_validation_outcomes(run_report.as_ref());

    let (blocked_reason_codes, blocked_truncated) = normalize_texts(blocked_reason_codes);
    let (reviewer_next_steps, reviewer_truncated) = normalize_texts(reviewer_next_steps);
    let (recommended_actions, recommended_truncated) = normalize_texts(recommended_actions);
    let (validation_outcomes, validation_truncated) =
        normalize_validation_outcomes(validation_outcomes);

    let feedback_signature = format!(
        "terminal={};blocked={};review={};recommended={};validation={}",
        terminal_state.as_deref().unwrap_or("unknown"),
        blocked_reason_codes.join("|"),
        reviewer_next_steps.join("|"),
        recommended_actions.join("|"),
        validation_outcomes
            .iter()
            .map(|outcome| format!(
                "{}:{}:{}",
                outcome.command,
                outcome.status,
                outcome
                    .exit_code
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "none".to_string())
            ))
            .collect::<Vec<_>>()
            .join("|")
    );

    Some(PlanningFeedbackArtifact {
        schema_version: PLANNING_FEEDBACK_SCHEMA_VERSION,
        source_run_id,
        terminal_state,
        blocked_reason_codes,
        reviewer_next_steps,
        recommended_actions,
        validation_outcomes,
        feedback_signature,
        truncated: blocked_truncated
            || reviewer_truncated
            || recommended_truncated
            || validation_truncated,
    })
}

fn read_run_report_feedback(path: Option<&Path>) -> Option<RunReportFeedbackCompat> {
    let path = path?;
    if !path.exists() {
        return None;
    }
    let raw = fs::read_to_string(path).ok()?;
    from_str(&raw).ok()
}

fn read_next_actions_feedback(path: Option<&Path>) -> Option<NextActionsArtifact> {
    let path = path?;
    if !path.exists() {
        return None;
    }
    load_next_actions(path).ok()
}

fn extract_validation_outcomes(
    report: Option<&RunReportFeedbackCompat>,
) -> Vec<PlanningValidationOutcome> {
    let Some(report) = report else {
        return Vec::new();
    };
    let mut outcomes = Vec::new();
    for execution in report.stage_executions.clone().unwrap_or_default() {
        if execution.stage != Stage::Validation || execution.status == StageExecutionStatus::Skipped
        {
            continue;
        }
        outcomes.push(PlanningValidationOutcome {
            command: execution.command,
            status: stage_execution_status_label(execution.status).to_string(),
            exit_code: execution.exit_code,
        });
    }
    outcomes
}

fn normalize_texts(items: Vec<String>) -> (Vec<String>, bool) {
    let mut values = items
        .into_iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(|value| truncate_chars(value, MAX_FEEDBACK_TEXT_LEN))
        .collect::<Vec<_>>();
    values.sort_unstable();
    values.dedup();
    let truncated = values.len() > MAX_FEEDBACK_ITEMS;
    if truncated {
        values.truncate(MAX_FEEDBACK_ITEMS);
    }
    (values, truncated)
}

fn normalize_validation_outcomes(
    items: Vec<PlanningValidationOutcome>,
) -> (Vec<PlanningValidationOutcome>, bool) {
    let mut keyed = items
        .into_iter()
        .map(|mut outcome| {
            outcome.command = truncate_chars(outcome.command, MAX_FEEDBACK_TEXT_LEN);
            let key = format!(
                "{}|{}|{}",
                outcome.command,
                outcome.status,
                outcome
                    .exit_code
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "none".to_string())
            );
            (key, outcome)
        })
        .collect::<Vec<_>>();
    keyed.sort_by(|a, b| a.0.cmp(&b.0));
    keyed.dedup_by(|a, b| a.0 == b.0);
    let truncated = keyed.len() > MAX_FEEDBACK_ITEMS;
    if truncated {
        keyed.truncate(MAX_FEEDBACK_ITEMS);
    }
    (
        keyed.into_iter().map(|(_, value)| value).collect(),
        truncated,
    )
}

fn truncate_chars(mut value: String, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value;
    }
    let mut truncated = String::new();
    for (index, ch) in value.chars().enumerate() {
        if index >= max_chars {
            break;
        }
        truncated.push(ch);
    }
    mem::swap(&mut value, &mut truncated);
    value
}

fn terminal_state_label(state: TerminalState) -> &'static str {
    match state {
        TerminalState::Done => "done",
        TerminalState::Blocked => "blocked",
        TerminalState::Failed => "failed",
        TerminalState::Timeout => "timeout",
    }
}

fn stage_execution_status_label(status: StageExecutionStatus) -> &'static str {
    match status {
        StageExecutionStatus::Success => "success",
        StageExecutionStatus::Skipped => "skipped",
        StageExecutionStatus::Failed => "failed",
        StageExecutionStatus::Timeout => "timeout",
        StageExecutionStatus::SpawnFailed => "spawn_failed",
        StageExecutionStatus::ArtifactMissing => "artifact_missing",
    }
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
