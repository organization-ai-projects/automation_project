use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::ProjectMetadata;

const ERR_READ_PROJECTS_DIR: &str = "Failed to read projects dir";
const ERR_READ_PROJECT_ENTRY: &str = "Failed to read project entry";
const ERR_READ_METADATA: &str = "Failed to read metadata.json";
const ERR_INVALID_METADATA: &str = "Invalid metadata.json";
const AUTO_DETECTED_DESCRIPTION: &str = "Auto-detected project";
const DEFAULT_VERSION: &str = "0.1.0";
const ERR_SERIALIZE_REGISTRY: &str = "Failed to serialize registry";
const ERR_WRITE_CACHE: &str = "Failed to write registry cache";
const METADATA_FILE: &str = "metadata.json";
const IGNORED_FOLDERS: [&str; 5] = [".git", "node_modules", "target", ".idea", ".vscode"];

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Registry {
    pub projects: HashMap<String, ProjectMetadata>,
}

impl Registry {
    /// Point d’entrée NORMAL du moteur.
    /// Scanne le dossier projets + hydrate le registry.
    pub fn load(projects_dir: impl AsRef<Path>) -> Result<Self, String> {
        let projects_dir = projects_dir.as_ref();

        if !projects_dir.is_dir() {
            return Ok(Self::default());
        }

        let mut registry = Self::default();
        registry.scan_projects(projects_dir)?;
        Ok(registry)
    }

    /// Scan automatique des projets présents sur disque
    pub fn scan_projects(&mut self, projects_dir: &Path) -> Result<(), String> {
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
            let key = folder.to_string(); // Use folder as the key
            if self.projects.insert(key.clone(), metadata).is_some() {
                warn!("Duplicate project key '{}', overwritten", key);
            }
        }

        Ok(())
    }

    /// Vérifie si un chemin est un dossier valide
    fn is_valid_project_dir(path: &Path) -> bool {
        if !path.is_dir() {
            return false;
        }
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        !IGNORED_FOLDERS.contains(&name) && !name.starts_with('.')
    }

    /// Charge metadata.json si présent, sinon fallback auto
    fn load_project_metadata(
        project_id: &str,
        project_dir: &Path,
    ) -> Result<ProjectMetadata, String> {
        let metadata_path = project_dir.join(METADATA_FILE);

        match fs::read_to_string(&metadata_path) {
            Ok(data) => {
                let mut meta: ProjectMetadata = serde_json::from_str(&data)
                    .map_err(|e| format!("{} in '{}': {}", ERR_INVALID_METADATA, project_id, e))?;

                // Log if metadata.json contains a mismatched ID
                if meta.id != project_id {
                    warn!(
                        "Mismatched ID in metadata.json: expected '{}', found '{}'",
                        project_id, meta.id
                    );
                }

                // Force meta.id to match project_id
                meta.id = project_id.to_string();
                if meta.name.trim().is_empty() {
                    meta.name = project_id.to_string();
                }
                if meta.description.trim().is_empty() {
                    meta.description = AUTO_DETECTED_DESCRIPTION.to_string();
                }
                if meta.version.trim().is_empty() {
                    meta.version = DEFAULT_VERSION.to_string();
                }

                Ok(meta)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Ok(Self::auto_metadata(project_id))
            }
            Err(e) => Err(format!(
                "{} '{}': {}",
                ERR_READ_METADATA,
                metadata_path.display(),
                e
            )),
        }
    }

    fn auto_metadata(project_id: &str) -> ProjectMetadata {
        ProjectMetadata {
            id: project_id.to_string(),
            name: project_id.to_string(),
            description: AUTO_DETECTED_DESCRIPTION.to_string(),
            version: DEFAULT_VERSION.to_string(),
        }
    }

    /// Sauvegarde optionnelle du cache (accélération startup)
    pub fn save_cache(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        let data = serde_json::to_string_pretty(self)
            .map_err(|e| format!("{}: {}", ERR_SERIALIZE_REGISTRY, e))?;
        fs::write(path, data)
            .map_err(|e| format!("{} '{}': {}", ERR_WRITE_CACHE, path.display(), e))?;
        Ok(())
    }

    // Future improvement: Add a `load_cache()` method to complement `save_cache()`
    // This could include invalidation logic based on directory modification times (mtime).
}
