//! Common test helpers and constants for versioning tests.
//!
//! This module provides shared utilities to reduce duplication across test files.

use crate::modification_category::ModificationCategory;
use crate::modification_entry::ModificationEntry;
use crate::release_id::ReleaseId;
use crate::revision_entry::RevisionEntry;
use crate::revision_log::RevisionLog;
use chrono::{DateTime, TimeZone, Utc};

// ============================================================================
// Common Test Constants
// ============================================================================

/// Standard project name used across tests
pub(crate) const TEST_PROJECT_NAME: &str = "TestProject";

/// Alternative project name for variety
pub(crate) const ALT_PROJECT_NAME: &str = "MyProject";

/// Common contributor names
pub(crate) const CONTRIBUTOR_ALICE: &str = "Alice";
pub(crate) const CONTRIBUTOR_BOB: &str = "Bob";
pub(crate) const CONTRIBUTOR_DEV_A: &str = "Developer A";
pub(crate) const CONTRIBUTOR_DEV_B: &str = "Developer B";
pub(crate) const CONTRIBUTOR_DEV_C: &str = "Developer C";

/// Common modification descriptions
pub(crate) const MOD_NEW_FEATURE: &str = "New feature";
pub(crate) const MOD_BUG_FIX: &str = "Bug fix";
pub(crate) const MOD_BREAKING_CHANGE: &str = "Breaking change";
pub(crate) const MOD_SECURITY_PATCH: &str = "Security patch";

// ============================================================================
// Helper Functions for ReleaseId
// ============================================================================

/// Creates a common test ReleaseId for version 1.2.3
pub(crate) fn release_id_1_2_3() -> ReleaseId {
    ReleaseId::build(1, 2, 3)
}

/// Creates a common test ReleaseId for version 2.0.0
pub(crate) fn release_id_2_0_0() -> ReleaseId {
    ReleaseId::build(2, 0, 0)
}

/// Creates a common test ReleaseId for version 1.0.0
pub(crate) fn release_id_1_0_0() -> ReleaseId {
    ReleaseId::build(1, 0, 0)
}

// ============================================================================
// Helper Functions for ModificationEntry
// ============================================================================

/// Creates a new feature modification with a custom description
pub(crate) fn new_feature_mod(description: impl Into<String>) -> ModificationEntry {
    ModificationEntry::create(description.into(), ModificationCategory::NewCapability)
}

/// Creates a bug fix modification with a custom description
pub(crate) fn bug_fix_mod(description: impl Into<String>) -> ModificationEntry {
    ModificationEntry::create(description.into(), ModificationCategory::CorrectionApplied)
}

/// Creates a breaking change modification with a custom description
pub(crate) fn breaking_mod(description: impl Into<String>) -> ModificationEntry {
    ModificationEntry::create(
        description.into(),
        ModificationCategory::BreakingModification,
    )
}

/// Creates an enhancement modification with a custom description
pub(crate) fn enhancement_mod(description: impl Into<String>) -> ModificationEntry {
    ModificationEntry::create(description.into(), ModificationCategory::Enhancement)
}

/// Creates a security update modification with a custom description
pub(crate) fn security_mod(description: impl Into<String>) -> ModificationEntry {
    ModificationEntry::create(description.into(), ModificationCategory::SecurityUpdate)
}

// ============================================================================
// Helper Functions for RevisionEntry
// ============================================================================

/// Creates a basic RevisionEntry with the given release ID
pub(crate) fn basic_revision_entry(release: ReleaseId) -> RevisionEntry {
    RevisionEntry::create(release, fixed_test_timestamp())
}

/// Fixed timestamp used by helpers to keep tests deterministic.
pub(crate) fn fixed_test_timestamp() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
        .single()
        .expect("valid fixed test timestamp")
}

/// Creates a RevisionEntry with modifications
pub(crate) fn revision_entry_with_mods(
    release: ReleaseId,
    modifications: Vec<ModificationEntry>,
) -> RevisionEntry {
    let mut entry = basic_revision_entry(release);
    for modification in modifications {
        entry.append_modification(modification);
    }
    entry
}

/// Creates a RevisionEntry with contributors
pub(crate) fn revision_entry_with_contributors(
    release: ReleaseId,
    contributors: Vec<String>,
) -> RevisionEntry {
    let mut entry = basic_revision_entry(release);
    for contributor in contributors {
        entry.append_contributor(contributor);
    }
    entry
}

// ============================================================================
// Helper Functions for RevisionLog
// ============================================================================

/// Creates a basic RevisionLog with the test project name
pub(crate) fn basic_revision_log() -> RevisionLog {
    RevisionLog::initialize(TEST_PROJECT_NAME.to_string())
}

/// Creates a RevisionLog with a custom project name
pub(crate) fn revision_log_with_name(name: &str) -> RevisionLog {
    RevisionLog::initialize(name.to_string())
}

/// Creates a RevisionLog with multiple entries
pub(crate) fn revision_log_with_entries(entries: Vec<RevisionEntry>) -> RevisionLog {
    let mut log = basic_revision_log();
    for entry in entries {
        log.append_entry(entry);
    }
    log
}
