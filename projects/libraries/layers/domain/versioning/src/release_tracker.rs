// projects/libraries/layers/domain/versioning/src/release_tracker.rs

use crate::modification_category::ModificationCategory;
use crate::modification_entry::ModificationEntry;
use crate::release_id::ReleaseId;
use crate::revision_entry::RevisionEntry;
use crate::revision_log::RevisionLog;
use chrono::Utc;
use common_json::json_error::JsonErrorCode;
use common_json::{JsonError, from_str, to_string_pretty};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Tracks and manages release versions without relying on Git
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseTracker {
    current_release: ReleaseId,
    revision_log: RevisionLog,
}

impl ReleaseTracker {
    /// Initialize a new tracker with project name
    pub fn initialize(project_name: String) -> Self {
        let initial_release = ReleaseId::initial();
        let mut revision_log = RevisionLog::initialize(project_name);

        let mut entry = RevisionEntry::create(initial_release, Utc::now());
        entry.append_modification(ModificationEntry::create(
            "Initial release".to_string(),
            ModificationCategory::NewCapability,
        ));
        revision_log.append_entry(entry);

        Self {
            current_release: initial_release,
            revision_log,
        }
    }

    /// Get the current release identifier
    pub fn active_release(&self) -> &ReleaseId {
        &self.current_release
    }

    /// Get the revision log
    pub fn log(&self) -> &RevisionLog {
        &self.revision_log
    }

    /// Register a new major release with changes
    pub fn register_major_release(
        &mut self,
        modifications: Vec<ModificationEntry>,
        contributors: Vec<String>,
    ) {
        self.current_release = self.current_release.advance_major();
        self.create_entry(modifications, contributors);
    }

    /// Register a new feature release with changes
    pub fn register_feature_release(
        &mut self,
        modifications: Vec<ModificationEntry>,
        contributors: Vec<String>,
    ) {
        self.current_release = self.current_release.advance_feature();
        self.create_entry(modifications, contributors);
    }

    /// Register a correction release with changes
    pub fn register_correction_release(
        &mut self,
        modifications: Vec<ModificationEntry>,
        contributors: Vec<String>,
    ) {
        self.current_release = self.current_release.advance_correction();
        self.create_entry(modifications, contributors);
    }

    fn create_entry(&mut self, modifications: Vec<ModificationEntry>, contributors: Vec<String>) {
        let mut entry = RevisionEntry::create(self.current_release, Utc::now());

        for modification in modifications {
            entry.append_modification(modification);
        }

        for contributor in contributors {
            entry.append_contributor(contributor);
        }

        self.revision_log.append_entry(entry);
    }

    /// Save tracker state to file
    pub fn persist_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), JsonError> {
        let serialized = to_string_pretty(self)?;
        fs::write(path, serialized).map_err(|e| {
            JsonError::new(JsonErrorCode::Io).context(format!("Failed to write file: {}", e))
        })
    }

    /// Load tracker state from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, JsonError> {
        let content = fs::read_to_string(path).map_err(|e| {
            JsonError::new(JsonErrorCode::Io).context(format!("Failed to read file: {}", e))
        })?;
        from_str(&content)
    }
}
