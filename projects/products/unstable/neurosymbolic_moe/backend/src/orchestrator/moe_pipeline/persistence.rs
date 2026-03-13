use crate::evaluation_engine::EvaluationEngine;
use crate::moe_core::{MoeError, TracePhase};
use crate::orchestrator::{
    ContinuousImprovementReport, GovernanceAuditTrail, GovernanceImportDecision,
    GovernancePersistenceBundle, GovernanceState, GovernanceStateDiff, GovernanceStateSnapshot,
    MoePipeline, RuntimePersistenceBundle,
};

impl MoePipeline {
    pub fn import_governance_state(&mut self, mut state: GovernanceState) {
        state.ensure_checksum();
        if !state.verify_checksum() {
            self.import_telemetry.record_governance_state_rejection();
            self.trace_logger.log_phase(
                crate::moe_core::TaskId::new("governance-import"),
                TracePhase::Validation,
                "governance state checksum mismatch during import".to_string(),
                None,
            );
            return;
        }

        let decision = self.evaluate_governance_import(&state);
        if !decision.allowed {
            self.import_telemetry.record_governance_state_rejection();
            self.trace_logger.log_phase(
                crate::moe_core::TaskId::new("governance-import"),
                TracePhase::Validation,
                format!(
                    "governance import rejected: {}",
                    decision.reasons.join("; ")
                ),
                None,
            );
            return;
        }

        self.continuous_governance_policy = state.continuous_governance_policy;
        self.evaluation_baseline = state.evaluation_baseline;
        self.last_continuous_improvement_report = state.last_continuous_improvement_report;
        self.governance_state_version = state.state_version;
        self.record_governance_audit("governance state imported");
        self.import_telemetry.record_governance_state_success();
    }

    pub fn export_governance_state_json(&self) -> Result<String, MoeError> {
        common_json::json::to_json_string_pretty(&self.export_governance_state()).map_err(|err| {
            MoeError::DatasetError(format!("governance state serialization failed: {err}"))
        })
    }

    pub fn export_governance_bundle(&self) -> GovernancePersistenceBundle {
        GovernancePersistenceBundle {
            state: self.export_governance_state(),
            audit_entries: self.governance_audit_entries.clone(),
            snapshots: self.governance_state_snapshots.clone(),
        }
    }

    pub fn export_governance_bundle_json(&self) -> Result<String, MoeError> {
        common_json::json::to_json_string_pretty(&self.export_governance_bundle()).map_err(|err| {
            MoeError::DatasetError(format!(
                "governance persistence bundle serialization failed: {err}"
            ))
        })
    }

    pub fn export_runtime_bundle(&self) -> RuntimePersistenceBundle {
        RuntimePersistenceBundle::from_components(
            self.export_governance_bundle(),
            self.short_term_memory.entries_cloned(),
            self.long_term_memory.entries_cloned(),
            self.buffer_manager.clone(),
        )
    }

    pub fn export_runtime_bundle_json(&self) -> Result<String, MoeError> {
        common_json::json::to_json_string_pretty(&self.export_runtime_bundle()).map_err(|err| {
            MoeError::DatasetError(format!(
                "runtime persistence bundle serialization failed: {err}"
            ))
        })
    }

    pub fn import_governance_state_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let payload_fingerprint =
            crate::orchestrator::import_journal::ImportJournal::payload_fingerprint(payload);
        if self
            .import_journal
            .has_successful_payload_fingerprint(&payload_fingerprint)
        {
            self.import_journal.record_deduplicated_replay();
            return Ok(());
        }
        let state = match Self::parse_governance_state_json_payload(payload) {
            Ok(state) => state,
            Err(err) => {
                if Self::is_json_parse_failure(&err) {
                    self.import_telemetry.record_json_parse_failure();
                    self.import_journal.record_parse_failure();
                }
                return Err(err);
            }
        };
        let result = self.try_import_governance_state(state);
        match result {
            Ok(()) => {
                self.import_journal
                    .record_successful_import(payload_fingerprint);
                Ok(())
            }
            Err(err) => {
                self.import_journal.record_rejection();
                Err(err)
            }
        }
    }

    pub fn import_governance_bundle(
        &mut self,
        bundle: GovernancePersistenceBundle,
    ) -> Result<(), MoeError> {
        let decision = self.evaluate_governance_bundle_import(&bundle)?;
        if let Err(err) = Self::ensure_import_allowed(&decision, "governance bundle rejected") {
            self.import_telemetry.record_governance_bundle_rejection();
            return Err(err);
        }

        self.continuous_governance_policy = bundle.state.continuous_governance_policy.clone();
        self.evaluation_baseline = bundle.state.evaluation_baseline.clone();
        self.last_continuous_improvement_report =
            bundle.state.last_continuous_improvement_report.clone();
        self.governance_state_version = bundle.state.state_version;

        self.governance_audit_entries = bundle.audit_entries;
        if self.governance_audit_entries.len() > self.max_governance_audit_entries {
            let to_trim = self.governance_audit_entries.len() - self.max_governance_audit_entries;
            self.governance_audit_entries.drain(0..to_trim);
        }

        self.governance_state_snapshots = bundle.snapshots;
        if self.governance_state_snapshots.len() > self.max_governance_state_snapshots {
            let to_trim =
                self.governance_state_snapshots.len() - self.max_governance_state_snapshots;
            self.governance_state_snapshots.drain(0..to_trim);
        }
        self.retain_snapshots_with_matching_audit_versions();
        self.import_telemetry.record_governance_bundle_success();

        Ok(())
    }

    pub fn compare_and_import_governance_bundle(
        &mut self,
        expected_current_version: u64,
        bundle: GovernancePersistenceBundle,
    ) -> Result<(), MoeError> {
        self.assert_expected_governance_state(expected_current_version, None)?;
        self.import_governance_bundle(bundle)
    }

    pub fn compare_and_import_governance_bundle_with_checksum(
        &mut self,
        expected_current_version: u64,
        expected_current_checksum: &str,
        bundle: GovernancePersistenceBundle,
    ) -> Result<(), MoeError> {
        self.assert_expected_governance_state(
            expected_current_version,
            Some(expected_current_checksum),
        )?;
        self.import_governance_bundle(bundle)
    }

    pub fn import_governance_bundle_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let payload_fingerprint =
            crate::orchestrator::import_journal::ImportJournal::payload_fingerprint(payload);
        if self
            .import_journal
            .has_successful_payload_fingerprint(&payload_fingerprint)
        {
            self.import_journal.record_deduplicated_replay();
            return Ok(());
        }
        let bundle = match Self::parse_governance_bundle_json_payload(payload) {
            Ok(bundle) => bundle,
            Err(err) => {
                if Self::is_json_parse_failure(&err) {
                    self.import_telemetry.record_json_parse_failure();
                    self.import_journal.record_parse_failure();
                }
                return Err(err);
            }
        };
        let result = self.import_governance_bundle(bundle);
        match result {
            Ok(()) => {
                self.import_journal
                    .record_successful_import(payload_fingerprint);
                Ok(())
            }
            Err(err) => {
                self.import_journal.record_rejection();
                Err(err)
            }
        }
    }

    pub fn compare_and_import_governance_bundle_json(
        &mut self,
        expected_current_version: u64,
        payload: &str,
    ) -> Result<(), MoeError> {
        Self::parse_and_apply_governance_bundle_json(payload, |bundle| {
            self.compare_and_import_governance_bundle(expected_current_version, bundle)
        })
    }

    pub fn compare_and_import_governance_bundle_json_with_checksum(
        &mut self,
        expected_current_version: u64,
        expected_current_checksum: &str,
        payload: &str,
    ) -> Result<(), MoeError> {
        Self::parse_and_apply_governance_bundle_json(payload, |bundle| {
            self.compare_and_import_governance_bundle_with_checksum(
                expected_current_version,
                expected_current_checksum,
                bundle,
            )
        })
    }

    pub fn import_runtime_bundle(
        &mut self,
        bundle: RuntimePersistenceBundle,
    ) -> Result<(), MoeError> {
        let decision = self.evaluate_runtime_bundle_import(&bundle)?;
        if let Err(err) = Self::ensure_import_allowed(&decision, "runtime bundle rejected") {
            self.import_telemetry.record_runtime_bundle_rejection();
            return Err(err);
        }

        // Apply runtime state atomically: if any future step becomes fallible, restore backup.
        let backup_governance_policy = self.continuous_governance_policy.clone();
        let backup_evaluation_baseline = self.evaluation_baseline.clone();
        let backup_last_report = self.last_continuous_improvement_report.clone();
        let backup_governance_state_version = self.governance_state_version;
        let backup_governance_audit_entries = self.governance_audit_entries.clone();
        let backup_governance_state_snapshots = self.governance_state_snapshots.clone();
        let backup_short_term_memory = self.short_term_memory.clone();
        let backup_long_term_memory = self.long_term_memory.clone();
        let backup_buffer_manager = self.buffer_manager.clone();

        let governance = bundle.governance;
        let short_term_memory_entries = bundle.short_term_memory_entries;
        let long_term_memory_entries = bundle.long_term_memory_entries;
        let buffer_manager = bundle.buffer_manager;

        if let Err(err) = (|| -> Result<(), MoeError> {
            self.import_governance_bundle(governance)?;
            self.short_term_memory
                .replace_entries(short_term_memory_entries)?;
            self.long_term_memory
                .replace_entries(long_term_memory_entries)?;
            self.buffer_manager = buffer_manager;
            Ok(())
        })() {
            self.continuous_governance_policy = backup_governance_policy;
            self.evaluation_baseline = backup_evaluation_baseline;
            self.last_continuous_improvement_report = backup_last_report;
            self.governance_state_version = backup_governance_state_version;
            self.governance_audit_entries = backup_governance_audit_entries;
            self.governance_state_snapshots = backup_governance_state_snapshots;
            self.short_term_memory = backup_short_term_memory;
            self.long_term_memory = backup_long_term_memory;
            self.buffer_manager = backup_buffer_manager;
            self.import_telemetry.record_runtime_bundle_rejection();
            return Err(MoeError::DatasetError(format!(
                "runtime bundle import failed and was rolled back: {err}"
            )));
        }

        self.import_telemetry.record_runtime_bundle_success();
        Ok(())
    }

    pub fn compare_and_import_runtime_bundle(
        &mut self,
        expected_current_version: u64,
        bundle: RuntimePersistenceBundle,
    ) -> Result<(), MoeError> {
        self.assert_expected_governance_state(expected_current_version, None)?;
        self.import_runtime_bundle(bundle)
    }

    pub fn compare_and_import_runtime_bundle_with_checksum(
        &mut self,
        expected_current_version: u64,
        expected_current_checksum: &str,
        bundle: RuntimePersistenceBundle,
    ) -> Result<(), MoeError> {
        self.assert_expected_governance_state(
            expected_current_version,
            Some(expected_current_checksum),
        )?;
        self.import_runtime_bundle(bundle)
    }

    pub fn import_runtime_bundle_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let payload_fingerprint =
            crate::orchestrator::import_journal::ImportJournal::payload_fingerprint(payload);
        if self
            .import_journal
            .has_successful_payload_fingerprint(&payload_fingerprint)
        {
            self.import_journal.record_deduplicated_replay();
            return Ok(());
        }
        let bundle = match Self::parse_runtime_bundle_json_payload(payload) {
            Ok(bundle) => bundle,
            Err(err) => {
                if Self::is_json_parse_failure(&err) {
                    self.import_telemetry.record_json_parse_failure();
                    self.import_journal.record_parse_failure();
                }
                return Err(err);
            }
        };
        let result = self.import_runtime_bundle(bundle);
        match result {
            Ok(()) => {
                self.import_journal
                    .record_successful_import(payload_fingerprint);
                Ok(())
            }
            Err(err) => {
                self.import_journal.record_rejection();
                Err(err)
            }
        }
    }

    pub fn compare_and_import_runtime_bundle_json(
        &mut self,
        expected_current_version: u64,
        payload: &str,
    ) -> Result<(), MoeError> {
        Self::parse_and_apply_runtime_bundle_json(payload, |bundle| {
            self.compare_and_import_runtime_bundle(expected_current_version, bundle)
        })
    }

    pub fn compare_and_import_runtime_bundle_json_with_checksum(
        &mut self,
        expected_current_version: u64,
        expected_current_checksum: &str,
        payload: &str,
    ) -> Result<(), MoeError> {
        Self::parse_and_apply_runtime_bundle_json(payload, |bundle| {
            self.compare_and_import_runtime_bundle_with_checksum(
                expected_current_version,
                expected_current_checksum,
                bundle,
            )
        })
    }

    pub fn try_import_runtime_bundle(
        &mut self,
        bundle: RuntimePersistenceBundle,
    ) -> Result<(), MoeError> {
        let decision = self.evaluate_runtime_bundle_import(&bundle)?;
        Self::ensure_import_allowed(&decision, "runtime bundle import rejected")?;
        self.import_runtime_bundle(bundle)
    }

    pub fn try_import_runtime_bundle_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let payload_fingerprint =
            crate::orchestrator::import_journal::ImportJournal::payload_fingerprint(payload);
        if self
            .import_journal
            .has_successful_payload_fingerprint(&payload_fingerprint)
        {
            self.import_journal.record_deduplicated_replay();
            return Ok(());
        }
        let result = Self::parse_and_apply_runtime_bundle_json(payload, |bundle| {
            self.try_import_runtime_bundle(bundle)
        });
        match result {
            Ok(()) => {
                self.import_journal
                    .record_successful_import(payload_fingerprint);
                Ok(())
            }
            Err(err) => {
                if Self::is_json_parse_failure(&err) {
                    self.import_journal.record_parse_failure();
                } else {
                    self.import_journal.record_rejection();
                }
                Err(err)
            }
        }
    }

    pub fn preview_runtime_bundle_import_json(
        &self,
        payload: &str,
    ) -> Result<GovernanceImportDecision, MoeError> {
        Self::parse_and_apply_runtime_bundle_json(payload, |bundle| {
            self.evaluate_runtime_bundle_import(&bundle)
        })
    }

    pub fn try_import_governance_bundle(
        &mut self,
        bundle: GovernancePersistenceBundle,
    ) -> Result<(), MoeError> {
        let decision = self.evaluate_governance_bundle_import(&bundle)?;
        Self::ensure_import_allowed(&decision, "governance bundle import rejected")?;

        self.import_governance_bundle(bundle)
    }

    pub fn try_import_governance_bundle_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let payload_fingerprint =
            crate::orchestrator::import_journal::ImportJournal::payload_fingerprint(payload);
        if self
            .import_journal
            .has_successful_payload_fingerprint(&payload_fingerprint)
        {
            self.import_journal.record_deduplicated_replay();
            return Ok(());
        }
        let result = Self::parse_and_apply_governance_bundle_json(payload, |bundle| {
            self.try_import_governance_bundle(bundle)
        });
        match result {
            Ok(()) => {
                self.import_journal
                    .record_successful_import(payload_fingerprint);
                Ok(())
            }
            Err(err) => {
                if Self::is_json_parse_failure(&err) {
                    self.import_journal.record_parse_failure();
                } else {
                    self.import_journal.record_rejection();
                }
                Err(err)
            }
        }
    }

    pub fn try_import_governance_state(&mut self, state: GovernanceState) -> Result<(), MoeError> {
        let state = match Self::verify_governance_state_checksum(state) {
            Ok(state) => state,
            Err(err) => {
                self.import_telemetry.record_governance_state_rejection();
                return Err(err);
            }
        };

        let decision = self.evaluate_governance_import(&state);
        if let Err(err) = Self::ensure_import_allowed(&decision, "governance import rejected") {
            self.import_telemetry.record_governance_state_rejection();
            return Err(err);
        }

        self.import_governance_state(state);
        Ok(())
    }

    fn is_json_parse_failure(err: &MoeError) -> bool {
        match err {
            MoeError::DatasetError(message) => message.contains("deserialization failed"),
            _ => false,
        }
    }

    pub fn compare_and_import_governance_state(
        &mut self,
        expected_current_version: u64,
        state: GovernanceState,
    ) -> Result<(), MoeError> {
        self.assert_expected_governance_state(expected_current_version, None)?;
        self.try_import_governance_state(state)
    }

    pub fn compare_and_import_governance_state_with_checksum(
        &mut self,
        expected_current_version: u64,
        expected_current_checksum: &str,
        state: GovernanceState,
    ) -> Result<(), MoeError> {
        self.assert_expected_governance_state(
            expected_current_version,
            Some(expected_current_checksum),
        )?;
        self.try_import_governance_state(state)
    }

    pub fn preview_governance_import_json(
        &self,
        payload: &str,
    ) -> Result<GovernanceImportDecision, MoeError> {
        let state = Self::parse_and_apply_governance_state_json(payload, |state| {
            Self::verify_governance_state_checksum(state)
        })?;
        Ok(self.evaluate_governance_import(&state))
    }

    pub fn preview_governance_bundle_import_json(
        &self,
        payload: &str,
    ) -> Result<GovernanceImportDecision, MoeError> {
        Self::parse_and_apply_governance_bundle_json(payload, |bundle| {
            self.evaluate_governance_bundle_import(&bundle)
        })
    }

    pub fn try_import_governance_state_json(&mut self, payload: &str) -> Result<(), MoeError> {
        let payload_fingerprint =
            crate::orchestrator::import_journal::ImportJournal::payload_fingerprint(payload);
        if self
            .import_journal
            .has_successful_payload_fingerprint(&payload_fingerprint)
        {
            self.import_journal.record_deduplicated_replay();
            return Ok(());
        }
        let result = Self::parse_and_apply_governance_state_json(payload, |state| {
            self.try_import_governance_state(state)
        });
        match result {
            Ok(()) => {
                self.import_journal
                    .record_successful_import(payload_fingerprint);
                Ok(())
            }
            Err(err) => {
                if Self::is_json_parse_failure(&err) {
                    self.import_journal.record_parse_failure();
                } else {
                    self.import_journal.record_rejection();
                }
                Err(err)
            }
        }
    }

    pub fn compare_and_import_governance_state_json(
        &mut self,
        expected_current_version: u64,
        payload: &str,
    ) -> Result<(), MoeError> {
        Self::parse_and_apply_governance_state_json(payload, |state| {
            self.compare_and_import_governance_state(expected_current_version, state)
        })
    }

    pub fn compare_and_import_governance_state_json_with_checksum(
        &mut self,
        expected_current_version: u64,
        expected_current_checksum: &str,
        payload: &str,
    ) -> Result<(), MoeError> {
        Self::parse_and_apply_governance_state_json(payload, |state| {
            self.compare_and_import_governance_state_with_checksum(
                expected_current_version,
                expected_current_checksum,
                state,
            )
        })
    }

    pub fn governance_audit_trail(&self) -> GovernanceAuditTrail {
        GovernanceAuditTrail {
            current_version: self.governance_state_version,
            current_checksum: self
                .governance_audit_entries
                .last()
                .map(|e| e.checksum.clone()),
            entries: self.governance_audit_entries.clone(),
        }
    }

    pub fn governance_state_snapshots(&self) -> &[GovernanceStateSnapshot] {
        &self.governance_state_snapshots
    }

    pub fn rollback_governance_state_to_version(&mut self, version: u64) -> Result<(), MoeError> {
        let snapshot = self
            .governance_state_snapshots
            .iter()
            .rev()
            .find(|snapshot| snapshot.version == version)
            .cloned()
            .ok_or_else(|| {
                MoeError::DatasetError(format!(
                    "governance rollback failed: snapshot version {} not found",
                    version
                ))
            })?;

        self.continuous_governance_policy = snapshot.state.continuous_governance_policy;
        self.evaluation_baseline = snapshot.state.evaluation_baseline;
        self.last_continuous_improvement_report = snapshot.state.last_continuous_improvement_report;
        self.governance_state_version = snapshot.state.state_version;
        self.record_governance_audit(&format!("governance rollback to version {}", version));
        Ok(())
    }

    pub fn diff_governance_state(&self, target: &GovernanceState) -> GovernanceStateDiff {
        let source = self.export_governance_state();

        let source_version = source.state_version;
        let target_version = target.state_version;
        let version_delta = target_version as i64 - source_version as i64;

        let source_policy_fp = source
            .continuous_governance_policy
            .as_ref()
            .map(|p| {
                format!(
                    "{:.6}:{:.6}:{:.6}:{:.6}:{}:{}",
                    p.min_expert_success_rate,
                    p.min_routing_accuracy,
                    p.low_score_threshold,
                    p.regression_drop_threshold,
                    p.block_on_human_review,
                    p.auto_promote_on_pass
                )
            })
            .unwrap_or_else(|| "-".to_string());
        let target_policy_fp = target
            .continuous_governance_policy
            .as_ref()
            .map(|p| {
                format!(
                    "{:.6}:{:.6}:{:.6}:{:.6}:{}:{}",
                    p.min_expert_success_rate,
                    p.min_routing_accuracy,
                    p.low_score_threshold,
                    p.regression_drop_threshold,
                    p.block_on_human_review,
                    p.auto_promote_on_pass
                )
            })
            .unwrap_or_else(|| "-".to_string());

        let source_baseline_fp = source
            .evaluation_baseline
            .as_ref()
            .map(EvaluationEngine::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());
        let target_baseline_fp = target
            .evaluation_baseline
            .as_ref()
            .map(EvaluationEngine::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());

        let source_report_fp = source
            .last_continuous_improvement_report
            .as_ref()
            .map(ContinuousImprovementReport::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());
        let target_report_fp = target
            .last_continuous_improvement_report
            .as_ref()
            .map(ContinuousImprovementReport::checksum_fingerprint)
            .unwrap_or_else(|| "-".to_string());

        let schema_version_changed = source.schema_version != target.schema_version;
        let checksum_changed = source.state_checksum != target.state_checksum;
        let policy_changed = source_policy_fp != target_policy_fp;
        let baseline_changed = source_baseline_fp != target_baseline_fp;
        let report_changed = source_report_fp != target_report_fp;

        let has_drift = schema_version_changed
            || checksum_changed
            || policy_changed
            || baseline_changed
            || report_changed
            || version_delta != 0;

        GovernanceStateDiff {
            source_version,
            target_version,
            version_delta,
            schema_version_changed,
            checksum_changed,
            policy_changed,
            baseline_changed,
            report_changed,
            has_drift,
        }
    }
}
