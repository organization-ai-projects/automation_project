use common_json::{from_str, to_string_pretty};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationInvocationArtifact {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoContextArtifact {
    repo_root: String,
    generated_at_unix_secs: u64,
    top_level_entries: Vec<String>,
    detected_validation_commands: Vec<ValidationInvocationArtifact>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LegacyOrCurrentValidationCommands {
    Legacy(Vec<String>),
    Current(Vec<ValidationInvocationArtifact>),
}

#[derive(Debug, Deserialize)]
struct RepoContextArtifactCompat {
    detected_validation_commands: LegacyOrCurrentValidationCommands,
}

pub fn write_repo_context_artifact(repo_root: &Path, artifact_path: &Path) -> Result<(), String> {
    let top_level_entries = list_top_level_entries(repo_root)?;
    let detected_validation_commands = detect_validation_commands(repo_root);
    let payload = RepoContextArtifact {
        repo_root: repo_root.display().to_string(),
        generated_at_unix_secs: unix_timestamp_secs(),
        top_level_entries,
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
        LegacyOrCurrentValidationCommands::Current(commands) => commands,
        LegacyOrCurrentValidationCommands::Legacy(commands) => commands
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
