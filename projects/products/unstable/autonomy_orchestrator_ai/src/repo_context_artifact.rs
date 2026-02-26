use common_json::{from_str, to_string_pretty};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::artifacts::{RepoContextArtifactCompat, ValidationInvocationArtifact};
use crate::versioning::VersioningCommands;

#[derive(Debug, Serialize, Deserialize)]
struct RepoContextArtifact {
    repo_root: String,
    generated_at_unix_secs: u64,
    top_level_entries: Vec<String>,
    workspace_members: Vec<String>,
    ownership_boundaries: Vec<String>,
    hot_paths: Vec<String>,
    detected_validation_commands: Vec<ValidationInvocationArtifact>,
}

pub fn write_repo_context_artifact(repo_root: &Path, artifact_path: &Path) -> Result<(), String> {
    let top_level_entries = list_top_level_entries(repo_root)?;
    let workspace_members = detect_workspace_members(repo_root);
    let ownership_boundaries = detect_ownership_boundaries(repo_root);
    let hot_paths = detect_hot_paths(repo_root);
    let detected_validation_commands = detect_validation_commands(repo_root);
    let payload = RepoContextArtifact {
        repo_root: repo_root.display().to_string(),
        generated_at_unix_secs: unix_timestamp_secs(),
        top_level_entries,
        workspace_members,
        ownership_boundaries,
        hot_paths,
        detected_validation_commands,
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
                    command: head.clone(),
                    args: tail.to_vec(),
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
            command: "cargo".to_string(),
            args: vec!["fmt", "--all", "--", "--check"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        });
        commands.push(ValidationInvocationArtifact {
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
        });
        commands.push(ValidationInvocationArtifact {
            command: "cargo".to_string(),
            args: vec!["test", "--workspace"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        });
    }
    if path_exists(repo_root, "package.json") {
        commands.push(ValidationInvocationArtifact {
            command: "npm".to_string(),
            args: vec!["run", "lint"]
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        });
        commands.push(ValidationInvocationArtifact {
            command: "npm".to_string(),
            args: vec!["test"].into_iter().map(ToString::to_string).collect(),
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

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
