use std::collections;

use crate::memory_engine::MemoryEntry;
use crate::moe_core::MoeError;
use crate::orchestrator::pipeline_moe::{
    MAX_GOVERNANCE_BUNDLE_JSON_BYTES, MAX_GOVERNANCE_STATE_JSON_BYTES,
    MAX_RUNTIME_BUNDLE_JSON_BYTES, MAX_RUNTIME_BUNDLE_SESSION_COUNT,
    MAX_RUNTIME_BUNDLE_SESSION_VALUES_TOTAL, MAX_RUNTIME_BUNDLE_TOTAL_MEMORY_ENTRIES,
    MAX_RUNTIME_BUNDLE_WORKING_ENTRIES,
};
use crate::orchestrator::{
    GovernanceAuditEntry, GovernanceImportDecision, GovernancePersistenceBundle, GovernanceState,
    GovernanceStateSnapshot, MoePipeline, RuntimePersistenceBundle, Version,
};

impl MoePipeline {
    pub(crate) fn evaluate_governance_bundle_import(
        &self,
        bundle: &GovernancePersistenceBundle,
    ) -> Result<GovernanceImportDecision, MoeError> {
        let mut state = bundle.state.clone();
        if !self
            .governance_runtime_state
            .governance_import_policy
            .allow_schema_change
            && !state.has_supported_schema()
        {
            return Err(MoeError::PolicyRejected(format!(
                "governance state schema version {} is not supported",
                state.schema_version
            )));
        }
        state.ensure_checksum();
        if !state.verify_checksum() {
            return Err(MoeError::PolicyRejected(
                "governance bundle checksum verification failed".to_string(),
            ));
        }
        if !self
            .governance_runtime_state
            .governance_import_policy
            .allow_schema_change
            && let Some(unsupported_snapshot) = bundle
                .snapshots
                .iter()
                .find(|snapshot| !snapshot.state.has_supported_schema())
        {
            return Err(MoeError::PolicyRejected(format!(
                "governance snapshot schema version {} is not supported",
                unsupported_snapshot.state.schema_version
            )));
        }
        if !bundle
            .snapshots
            .iter()
            .all(|snapshot| snapshot.state.verify_checksum())
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle checksum verification failed".to_string(),
            ));
        }

        let mut normalized_bundle = bundle.clone();
        normalized_bundle.state = state;
        Self::validate_governance_bundle_consistency(&normalized_bundle)?;
        Ok(self.evaluate_governance_import(&normalized_bundle.state))
    }
    pub(crate) fn evaluate_runtime_bundle_import(
        &self,
        bundle: &RuntimePersistenceBundle,
    ) -> Result<GovernanceImportDecision, MoeError> {
        if !self
            .governance_runtime_state
            .governance_import_policy
            .allow_schema_change
            && !bundle.has_supported_schema()
        {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle schema version {} is not supported",
                bundle.schema_version
            )));
        }
        if !bundle.verify_checksum() {
            return Err(MoeError::PolicyRejected(
                "runtime bundle checksum verification failed".to_string(),
            ));
        }
        Self::validate_runtime_bundle_consistency(bundle)?;
        self.evaluate_governance_bundle_import(&bundle.governance)
    }
    pub(crate) fn validate_runtime_bundle_consistency(
        bundle: &RuntimePersistenceBundle,
    ) -> Result<(), MoeError> {
        let short_duplicates = Self::duplicate_memory_ids(&bundle.short_term_memory_entries);
        if !short_duplicates.is_empty() {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle rejected: duplicate short-term memory ids: {}",
                short_duplicates.join(", ")
            )));
        }

        let long_duplicates = Self::duplicate_memory_ids(&bundle.long_term_memory_entries);
        if !long_duplicates.is_empty() {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle rejected: duplicate long-term memory ids: {}",
                long_duplicates.join(", ")
            )));
        }

        let short_ids: std::collections::HashSet<&str> = bundle
            .short_term_memory_entries
            .iter()
            .map(|entry| entry.id.as_str())
            .collect();
        let mut overlap_ids: Vec<&str> = bundle
            .long_term_memory_entries
            .iter()
            .map(|entry| entry.id.as_str())
            .filter(|id| short_ids.contains(id))
            .collect();
        overlap_ids.sort_unstable();
        overlap_ids.dedup();
        if !overlap_ids.is_empty() {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle rejected: memory ids overlap between short and long term: {}",
                overlap_ids.join(", ")
            )));
        }

        let total_memory_entries =
            bundle.short_term_memory_entries.len() + bundle.long_term_memory_entries.len();
        if total_memory_entries > MAX_RUNTIME_BUNDLE_TOTAL_MEMORY_ENTRIES {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle rejected: too many memory entries ({} > {})",
                total_memory_entries, MAX_RUNTIME_BUNDLE_TOTAL_MEMORY_ENTRIES
            )));
        }

        let working_entries = bundle.buffer_manager.working().count();
        if working_entries > MAX_RUNTIME_BUNDLE_WORKING_ENTRIES {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle rejected: too many working buffer entries ({} > {})",
                working_entries, MAX_RUNTIME_BUNDLE_WORKING_ENTRIES
            )));
        }

        let sessions = bundle.buffer_manager.sessions().list_sessions();
        if sessions.len() > MAX_RUNTIME_BUNDLE_SESSION_COUNT {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle rejected: too many sessions ({} > {})",
                sessions.len(),
                MAX_RUNTIME_BUNDLE_SESSION_COUNT
            )));
        }
        let total_session_values: usize = sessions
            .iter()
            .map(|session| bundle.buffer_manager.sessions().values(session).len())
            .sum();
        if total_session_values > MAX_RUNTIME_BUNDLE_SESSION_VALUES_TOTAL {
            return Err(MoeError::PolicyRejected(format!(
                "runtime bundle rejected: too many session buffer values ({} > {})",
                total_session_values, MAX_RUNTIME_BUNDLE_SESSION_VALUES_TOTAL
            )));
        }

        Ok(())
    }
    pub(crate) fn duplicate_memory_ids(entries: &[MemoryEntry]) -> Vec<String> {
        let mut seen: collections::HashSet<&str> = collections::HashSet::new();
        let mut duplicates: Vec<String> = entries
            .iter()
            .map(|entry| entry.id.as_str())
            .filter(|id| !seen.insert(*id))
            .map(ToString::to_string)
            .collect();
        duplicates.sort();
        duplicates.dedup();
        duplicates
    }
    pub(crate) fn current_governance_checksum(&self) -> String {
        self.governance_runtime_state
            .governance_audit_entries
            .last()
            .map(|entry| entry.checksum.clone())
            .unwrap_or_else(|| self.export_governance_state().state_checksum)
    }
    pub(crate) fn ensure_import_allowed(
        decision: &GovernanceImportDecision,
        rejection_prefix: &str,
    ) -> Result<(), MoeError> {
        if decision.allowed {
            Ok(())
        } else {
            Err(MoeError::PolicyRejected(format!(
                "{rejection_prefix}: {}",
                decision.reasons.join("; ")
            )))
        }
    }
    pub(crate) fn verify_governance_state_checksum(
        mut state: GovernanceState,
    ) -> Result<GovernanceState, MoeError> {
        state.ensure_checksum();
        if state.verify_checksum() {
            Ok(state)
        } else {
            Err(MoeError::PolicyRejected(
                "governance state checksum verification failed".to_string(),
            ))
        }
    }
    pub(crate) fn parse_and_apply_governance_bundle_json<T, F>(
        payload: &str,
        operation: F,
    ) -> Result<T, MoeError>
    where
        F: FnOnce(GovernancePersistenceBundle) -> Result<T, MoeError>,
    {
        let bundle = Self::parse_governance_bundle_json_payload(payload)?;
        operation(bundle)
    }
    pub(crate) fn parse_and_apply_runtime_bundle_json<T, F>(
        payload: &str,
        operation: F,
    ) -> Result<T, MoeError>
    where
        F: FnOnce(RuntimePersistenceBundle) -> Result<T, MoeError>,
    {
        let bundle = Self::parse_runtime_bundle_json_payload(payload)?;
        operation(bundle)
    }
    pub(crate) fn parse_and_apply_governance_state_json<T, F>(
        payload: &str,
        operation: F,
    ) -> Result<T, MoeError>
    where
        F: FnOnce(GovernanceState) -> Result<T, MoeError>,
    {
        let state = Self::parse_governance_state_json_payload(payload)?;
        operation(state)
    }
    pub(crate) fn parse_governance_bundle_json_payload(
        payload: &str,
    ) -> Result<GovernancePersistenceBundle, MoeError> {
        if payload.len() > MAX_GOVERNANCE_BUNDLE_JSON_BYTES {
            return Err(MoeError::PolicyRejected(format!(
                "governance persistence bundle payload too large ({} bytes > {} bytes)",
                payload.len(),
                MAX_GOVERNANCE_BUNDLE_JSON_BYTES
            )));
        }
        common_json::json::from_json_str(payload).map_err(|err| {
            MoeError::DatasetError(format!(
                "governance persistence bundle deserialization failed: {err}"
            ))
        })
    }
    pub(crate) fn parse_governance_state_json_payload(
        payload: &str,
    ) -> Result<GovernanceState, MoeError> {
        if payload.len() > MAX_GOVERNANCE_STATE_JSON_BYTES {
            return Err(MoeError::PolicyRejected(format!(
                "governance state payload too large ({} bytes > {} bytes)",
                payload.len(),
                MAX_GOVERNANCE_STATE_JSON_BYTES
            )));
        }
        let mut state: GovernanceState =
            common_json::json::from_json_str(payload).map_err(|err| {
                MoeError::DatasetError(format!("governance state deserialization failed: {err}"))
            })?;
        state.ensure_checksum();
        Ok(state)
    }
    pub(crate) fn parse_runtime_bundle_json_payload(
        payload: &str,
    ) -> Result<RuntimePersistenceBundle, MoeError> {
        if payload.len() > MAX_RUNTIME_BUNDLE_JSON_BYTES {
            return Err(MoeError::PolicyRejected(format!(
                "runtime persistence bundle payload too large ({} bytes > {} bytes)",
                payload.len(),
                MAX_RUNTIME_BUNDLE_JSON_BYTES
            )));
        }
        let mut bundle: RuntimePersistenceBundle = common_json::json::from_json_str(payload)
            .map_err(|err| {
                MoeError::DatasetError(format!(
                    "runtime persistence bundle deserialization failed: {err}"
                ))
            })?;
        bundle.ensure_checksum();
        Ok(bundle)
    }
    pub(crate) fn assert_expected_governance_state(
        &self,
        expected_current_version: Version,
        expected_current_checksum: Option<&str>,
    ) -> Result<(), MoeError> {
        if self.governance_runtime_state.governance_state_version != expected_current_version {
            return Err(MoeError::PolicyRejected(format!(
                "compare-and-import rejected: expected governance version {}, current version {}",
                expected_current_version, self.governance_runtime_state.governance_state_version
            )));
        }
        if let Some(expected_checksum) = expected_current_checksum {
            let current_checksum = self.current_governance_checksum();
            if current_checksum != expected_checksum {
                return Err(MoeError::PolicyRejected(format!(
                    "compare-and-import rejected: expected governance checksum {}, current checksum {}",
                    expected_checksum, current_checksum
                )));
            }
        }
        Ok(())
    }
    pub(crate) fn validate_governance_bundle_consistency(
        bundle: &GovernancePersistenceBundle,
    ) -> Result<(), MoeError> {
        if bundle
            .audit_entries
            .windows(2)
            .any(|pair| pair[0].version >= pair[1].version)
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: audit versions must be strictly increasing"
                    .to_string(),
            ));
        }

        if bundle
            .snapshots
            .windows(2)
            .any(|pair| pair[0].version >= pair[1].version)
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: snapshot versions must be strictly increasing"
                    .to_string(),
            ));
        }

        if let Some(last_audit) = bundle.audit_entries.last() {
            if last_audit.version != bundle.state.version_number {
                return Err(MoeError::PolicyRejected(
                    "governance bundle rejected: latest audit version does not match state version"
                        .to_string(),
                ));
            }
            if last_audit.checksum != bundle.state.state_checksum {
                return Err(MoeError::PolicyRejected(
                    "governance bundle rejected: latest audit checksum does not match state checksum"
                        .to_string(),
                ));
            }
        }

        if bundle
            .snapshots
            .iter()
            .any(|snapshot| snapshot.version != snapshot.state.version_number)
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: snapshot version does not match embedded state version"
                    .to_string(),
            ));
        }

        if let Some(last_snapshot) = bundle.snapshots.last() {
            if last_snapshot.version != bundle.state.version_number {
                return Err(MoeError::PolicyRejected(
                    "governance bundle rejected: latest snapshot version does not match state version"
                        .to_string(),
                ));
            }
            if last_snapshot.state.state_checksum != bundle.state.state_checksum {
                return Err(MoeError::PolicyRejected(
                    "governance bundle rejected: latest snapshot checksum does not match state checksum"
                        .to_string(),
                ));
            }
        }

        if let (Some(last_audit), Some(last_snapshot)) =
            (bundle.audit_entries.last(), bundle.snapshots.last())
            && last_audit.version != last_snapshot.version
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: latest audit and snapshot versions diverge"
                    .to_string(),
            ));
        }

        let snapshot_checksums_by_version: collections::HashMap<Version, &str> = bundle
            .snapshots
            .iter()
            .map(|snapshot| {
                (
                    snapshot.version.clone(),
                    snapshot.state.state_checksum.as_str(),
                )
            })
            .collect();

        let audit_versions: collections::HashSet<Version> = bundle
            .audit_entries
            .iter()
            .map(|entry| entry.version.clone())
            .collect();
        if bundle
            .snapshots
            .iter()
            .any(|snapshot| !audit_versions.contains(&snapshot.version))
        {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: snapshot version missing matching audit entry"
                    .to_string(),
            ));
        }

        if bundle.audit_entries.iter().any(|audit| {
            snapshot_checksums_by_version
                .get(&audit.version)
                .is_some_and(|snapshot_checksum| *snapshot_checksum != audit.checksum.as_str())
        }) {
            return Err(MoeError::PolicyRejected(
                "governance bundle rejected: audit checksum diverges from snapshot checksum for same version"
                    .to_string(),
            ));
        }

        Ok(())
    }
    pub(crate) fn record_governance_audit(&mut self, reason: &str) {
        self.governance_runtime_state
            .governance_state_version
            .increment_patch();
        let state = self.export_governance_state();
        let checksum = state.state_checksum.clone();
        self.governance_runtime_state
            .governance_audit_entries
            .push(GovernanceAuditEntry {
                version: self
                    .governance_runtime_state
                    .governance_state_version
                    .clone(),
                checksum,
                reason: reason.to_string(),
            });
        if self.governance_runtime_state.governance_audit_entries.len()
            > self.governance_runtime_state.max_governance_audit_entries
        {
            let to_trim = self.governance_runtime_state.governance_audit_entries.len()
                - self.governance_runtime_state.max_governance_audit_entries;
            self.governance_runtime_state
                .governance_audit_entries
                .drain(0..to_trim);
        }

        self.governance_runtime_state
            .governance_state_snapshots
            .push(GovernanceStateSnapshot {
                version: self
                    .governance_runtime_state
                    .governance_state_version
                    .clone(),
                reason: reason.to_string(),
                state,
            });
        if self
            .governance_runtime_state
            .governance_state_snapshots
            .len()
            > self.governance_runtime_state.max_governance_state_snapshots
        {
            let to_trim = self
                .governance_runtime_state
                .governance_state_snapshots
                .len()
                - self.governance_runtime_state.max_governance_state_snapshots;
            self.governance_runtime_state
                .governance_state_snapshots
                .drain(0..to_trim);
        }
        self.retain_snapshots_with_matching_audit_versions();
    }
    pub(crate) fn evaluate_governance_import(
        &self,
        state: &GovernanceState,
    ) -> GovernanceImportDecision {
        let diff = self.diff_governance_state(state);
        let mut reasons = Vec::new();

        if !self
            .governance_runtime_state
            .governance_import_policy
            .allow_schema_change
            && diff.schema_version_changed
        {
            reasons.push("schema version drift is not allowed".to_string());
        }
        if !self
            .governance_runtime_state
            .governance_import_policy
            .allow_version_regression
            && diff.version_delta.is_regression()
        {
            reasons.push("version regression is not allowed".to_string());
        }
        if let Some(max) = &self
            .governance_runtime_state
            .governance_import_policy
            .max_version_regression
            && diff.version_delta.is_regression()
            && diff.version_delta.exceeds_limit(max)
        {
            reasons.push("version regression exceeds configured maximum".to_string());
        }
        if self
            .governance_runtime_state
            .governance_import_policy
            .require_policy_match
            && diff.policy_changed
        {
            reasons.push("governance policy mismatch".to_string());
        }

        GovernanceImportDecision {
            allowed: reasons.is_empty(),
            reasons,
            diff,
        }
    }
    pub(crate) fn retain_snapshots_with_matching_audit_versions(&mut self) {
        if self
            .governance_runtime_state
            .governance_state_snapshots
            .is_empty()
            || self
                .governance_runtime_state
                .governance_audit_entries
                .is_empty()
        {
            self.governance_runtime_state
                .governance_state_snapshots
                .clear();
            return;
        }

        let audit_versions: std::collections::HashSet<Version> = self
            .governance_runtime_state
            .governance_audit_entries
            .iter()
            .map(|entry| entry.version.clone())
            .collect();
        self.governance_runtime_state
            .governance_state_snapshots
            .retain(|snapshot| audit_versions.contains(&snapshot.version));
    }
}
