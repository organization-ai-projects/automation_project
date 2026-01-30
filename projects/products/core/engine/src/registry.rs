// projects/products/core/engine/src/registry.rs
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use common_json::JsonSerializable;
use common_time::timestamp_utils::current_timestamp_ms;
use protocol::ProjectMetadata;
use protocol::protocol_id::ProtocolId;
use serde::{Deserialize, Serialize};
use tracing::warn;

const ERR_READ_PROJECTS_DIR: &str = "Failed to read projects dir";
const ERR_READ_PROJECT_ENTRY: &str = "Failed to read project entry";
const ERR_READ_METADATA: &str = "Failed to read metadata.ron";
const DEFAULT_VERSION: &str = "0.1.0";
const DEFAULT_KIND: &str = "product";
const _ERR_SERIALIZE_REGISTRY: &str = "Failed to serialize registry";
const ERR_WRITE_CACHE: &str = "Failed to write registry cache";
const METADATA_FILE: &str = "metadata.ron";
const IGNORED_FOLDERS: [&str; 5] = [".git", "node_modules", "target", ".idea", ".vscode"];

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub(crate) struct Registry {
    pub(crate) projects: HashMap<String, ProjectMetadata>,
}

impl Registry {
    /// NORMAL entry point of the engine.
    /// Scans the projects folder and populates the registry.
    pub(crate) fn load(projects_dir: impl AsRef<Path>) -> Result<Self, String> {
        let projects_dir = projects_dir.as_ref();

        if !projects_dir.is_dir() {
            return Ok(Self::default());
        }

        let mut registry = Self::default();
        registry.scan_projects(projects_dir)?;
        Ok(registry)
    }

    /// Automatic scan of projects present on disk
    pub(crate) fn scan_projects(&mut self, projects_dir: &Path) -> Result<(), String> {
        let entries = fs::read_dir(projects_dir).map_err(|e| {
            format!(
                "{} '{}': {}",
                ERR_READ_PROJECTS_DIR,
                projects_dir.display(),
                e
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("{}: {}", ERR_READ_PROJECT_ENTRY, e))?;
            let path = entry.path();

            if !Self::is_valid_project_dir(&path) {
                continue;
            }

            let Some(folder) = path
                .file_name()
                .map(|s| s.to_string_lossy())
                .filter(|s| !s.trim().is_empty())
            else {
                continue;
            };

            let metadata = Self::load_project_metadata(&folder, &path)?;
            let key = folder.to_string();
            if self.projects.insert(key.clone(), metadata).is_some() {
                warn!("Duplicate project key '{}', overwritten", key);
            }
        }

        Ok(())
    }

    /// Checks if a path is a valid directory
    fn is_valid_project_dir(path: &Path) -> bool {
        if !path.is_dir() {
            return false;
        }
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        !IGNORED_FOLDERS.contains(&name) && !name.starts_with('.')
    }

    /// Loads metadata.ron if present, otherwise auto fallback
    fn load_project_metadata(
        project_id: &str,
        project_dir: &Path,
    ) -> Result<ProjectMetadata, String> {
        let metadata_path = project_dir.join(METADATA_FILE);

        match fs::read_to_string(&metadata_path) {
            Ok(data) => {
                let mut meta: ProjectMetadata = ron::from_str(&data)
                    .map_err(|e| format!("Failed to parse metadata.ron: {e}"))?;

                if let Ok(expected) = ProtocolId::from_str(project_id) {
                    if meta.id != expected {
                        warn!(
                            "Mismatched ID in metadata.ron: expected '{}', found '{}'",
                            project_id, meta.id
                        );
                    }
                } else {
                    return Err(format!(
                        "Project folder '{}' must be a ProtocolId hex",
                        project_id
                    ));
                }

                if meta.name.trim().is_empty() {
                    meta.name = project_id.to_string();
                }
                if meta.kind.trim().is_empty() {
                    meta.kind = DEFAULT_KIND.to_string();
                }
                if meta.version.trim().is_empty() {
                    meta.version = DEFAULT_VERSION.to_string();
                }

                Ok(meta)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Self::auto_metadata(project_id),
            Err(e) => Err(format!(
                "{} '{}': {}",
                ERR_READ_METADATA,
                metadata_path.display(),
                e
            )),
        }
    }

    fn auto_metadata(project_id: &str) -> Result<ProjectMetadata, String> {
        let id = ProtocolId::from_str(project_id)
            .map_err(|e| format!("project.id must be a ProtocolId hex: {e}"))?;
        Ok(ProjectMetadata {
            schema_version: 1,
            generated_at: current_timestamp_ms(),
            id,
            name: project_id.to_string(),
            kind: DEFAULT_KIND.to_string(),
            version: DEFAULT_VERSION.to_string(),
            entrypoints: None,
            capabilities: Vec::new(),
            domains: Vec::new(),
            ai_hints: None,
        })
    }

    /// Optional cache saving (startup acceleration)
    pub(crate) fn save_cache(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        let data = self.to_json_string().unwrap_or_else(|e| {
            panic!("Failed to serialize ProjectMetadata: {e}");
        });
        fs::write(path, data)
            .map_err(|e| format!("{} '{}': {}", ERR_WRITE_CACHE, path.display(), e))?;
        Ok(())
    }

    // Future improvement: Add a `load_cache()` method to complement `save_cache()`
    // This could include invalidation logic based on directory modification times (mtime).
}
