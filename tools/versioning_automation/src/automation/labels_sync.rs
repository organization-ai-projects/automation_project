use std::collections::BTreeSet;
use std::fs;

use crate::automation::commands::LabelsSyncOptions;

use super::execute::{object_string, parse_json_array, run_gh_output, run_gh_status};

pub(crate) fn run_labels_sync(opts: LabelsSyncOptions) -> Result<(), String> {
    let content = fs::read_to_string(&opts.labels_file).map_err(|e| {
        format!(
            "Labels file not found or unreadable '{}': {e}",
            opts.labels_file
        )
    })?;
    let labels = parse_labels(&content, &opts.labels_file)?;

    let existing = run_gh_output(&[
        "label", "list", "--limit", "1000", "--json", "name", "--jq", ".[].name",
    ])?;
    let mut existing_set: BTreeSet<String> = existing
        .lines()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .collect();

    for (name, color, description) in &labels {
        if name.trim().is_empty() {
            return Err("Label missing field 'name'".to_string());
        }
        if color.trim().is_empty() {
            return Err(format!("Label '{name}' missing field 'color'"));
        }

        if existing_set.contains(name) {
            run_gh_status(&[
                "label",
                "edit",
                name,
                "--color",
                color,
                "--description",
                description,
            ])?;
        } else {
            run_gh_status(&[
                "label",
                "create",
                name,
                "--color",
                color,
                "--description",
                description,
            ])?;
            existing_set.insert(name.clone());
        }
    }

    if opts.prune {
        let desired: BTreeSet<String> = labels
            .iter()
            .map(|(name, _, _)| name.clone())
            .filter(|name| !name.trim().is_empty())
            .collect();

        let repo_labels = run_gh_output(&[
            "label", "list", "--limit", "1000", "--json", "name", "--jq", ".[].name",
        ])?;
        let protected: BTreeSet<String> = [
            "bug",
            "documentation",
            "duplicate",
            "enhancement",
            "good first issue",
            "help wanted",
            "invalid",
            "question",
            "wontfix",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        for label in repo_labels
            .lines()
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
        {
            if desired.contains(label) || protected.contains(label) {
                continue;
            }
            let _ = run_gh_status(&["label", "delete", label, "--yes"]);
        }
    }

    Ok(())
}

fn parse_labels(content: &str, source_name: &str) -> Result<Vec<(String, String, String)>, String> {
    let parsed = parse_json_array(content, &format!("labels JSON '{source_name}'"))?;
    let mut labels = Vec::with_capacity(parsed.len());
    for label in parsed {
        let Some(object) = label.as_object() else {
            return Err(format!(
                "Invalid label entry in '{source_name}': expected object"
            ));
        };
        labels.push((
            object_string(object, "name"),
            object_string(object, "color"),
            object_string(object, "description"),
        ));
    }
    Ok(labels)
}
