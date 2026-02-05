// projects/libraries/versioning/src/revision_log.rs

use crate::release_id::ReleaseId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single modification entry in the revision log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModificationEntry {
    description: String,
    category: ModificationCategory,
}

impl ModificationEntry {
    pub fn create(description: String, category: ModificationCategory) -> Self {
        Self {
            description,
            category,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_category(&self) -> &ModificationCategory {
        &self.category
    }
}

/// Categories for different types of modifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModificationCategory {
    BreakingModification,
    NewCapability,
    Enhancement,
    CorrectionApplied,
    SecurityUpdate,
    DeprecationNotice,
}

impl ModificationCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::BreakingModification => "Breaking Change",
            Self::NewCapability => "New Feature",
            Self::Enhancement => "Improvement",
            Self::CorrectionApplied => "Fix",
            Self::SecurityUpdate => "Security",
            Self::DeprecationNotice => "Deprecated",
        }
    }
}

/// A collection of modifications for a specific release
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionEntry {
    release: ReleaseId,
    timestamp: DateTime<Utc>,
    modifications: Vec<ModificationEntry>,
    contributors: Vec<String>,
}

impl RevisionEntry {
    pub fn create(release: ReleaseId, timestamp: DateTime<Utc>) -> Self {
        Self {
            release,
            timestamp,
            modifications: Vec::new(),
            contributors: Vec::new(),
        }
    }

    pub fn append_modification(&mut self, entry: ModificationEntry) {
        self.modifications.push(entry);
    }

    pub fn append_contributor(&mut self, name: String) {
        if !self.contributors.contains(&name) {
            self.contributors.push(name);
        }
    }

    pub fn get_release(&self) -> &ReleaseId {
        &self.release
    }

    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn get_modifications(&self) -> &[ModificationEntry] {
        &self.modifications
    }

    pub fn get_contributors(&self) -> &[String] {
        &self.contributors
    }
}

/// Manages the complete revision history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionLog {
    entries: Vec<RevisionEntry>,
    project_title: String,
}

impl RevisionLog {
    pub fn initialize(project_title: String) -> Self {
        Self {
            entries: Vec::new(),
            project_title,
        }
    }

    pub fn append_entry(&mut self, entry: RevisionEntry) {
        self.entries.push(entry);
        // Keep sorted by release ID (descending)
        self.entries.sort_by(|a, b| b.release.cmp(&a.release));
    }

    pub fn get_entries(&self) -> &[RevisionEntry] {
        &self.entries
    }

    pub fn get_project_title(&self) -> &str {
        &self.project_title
    }

    pub fn find_entry(&self, release: &ReleaseId) -> Option<&RevisionEntry> {
        self.entries.iter().find(|e| e.release == *release)
    }

    pub fn most_recent(&self) -> Option<&RevisionEntry> {
        self.entries.first()
    }
}
