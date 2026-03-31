//! tools/versioning_automation/src/automation/commands/labels_sync_options.rs
use std::{collections::BTreeSet, fs};

use crate::{
    automation::execute::{object_string, parse_json_array},
    gh_cli,
};

#[derive(Debug)]
pub(crate) struct LabelsSyncOptions {
    pub(crate) labels_file: String,
    pub(crate) prune: bool,
}

impl LabelsSyncOptions {
    pub(crate) fn run_labels_sync(self) -> Result<(), String> {
        let content = fs::read_to_string(&self.labels_file).map_err(|e| {
            format!(
                "Labels file not found or unreadable '{}': {e}",
                self.labels_file
            )
        })?;
        let labels = Self::parse_labels(&content, &self.labels_file)?;

        let existing = gh_cli::output_trim(&[
            "label", "list", "--limit", "1000", "--json", "name", "--jq", ".[].name",
        ])
        .map_err(|e| format!("Failed to run gh label list: {e}"))?;
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
                gh_cli::status(&[
                    "label",
                    "edit",
                    name,
                    "--color",
                    color,
                    "--description",
                    description,
                ])
                .map_err(|e| format!("Failed to run gh label edit: {e}"))?;
            } else {
                gh_cli::status(&[
                    "label",
                    "create",
                    name,
                    "--color",
                    color,
                    "--description",
                    description,
                ])
                .map_err(|e| format!("Failed to run gh label create: {e}"))?;
                existing_set.insert(name.clone());
            }
        }

        if self.prune {
            let desired: BTreeSet<String> = labels
                .iter()
                .map(|(name, _, _)| name.clone())
                .filter(|name| !name.trim().is_empty())
                .collect();

            let repo_labels = gh_cli::output_trim(&[
                "label", "list", "--limit", "1000", "--json", "name", "--jq", ".[].name",
            ])
            .map_err(|e| format!("Failed to run gh label list: {e}"))?;
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
                let _ = gh_cli::status(&["label", "delete", label, "--yes"]);
            }
        }

        Ok(())
    }

    fn parse_labels(
        content: &str,
        source_name: &str,
    ) -> Result<Vec<(String, String, String)>, String> {
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
}
