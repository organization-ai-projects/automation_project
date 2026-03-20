//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/training_runtime_state.rs
use crate::dataset_engine::{DatasetStore, TraceConverter};
use crate::feedback_engine::FeedbackStore;
use crate::moe_core::MoeError;
use crate::orchestrator::{AutoImprovementPolicy, AutoImprovementStatus, ModelRegistry};

#[derive(Clone)]
pub(in crate::orchestrator) struct TrainingRuntimeState {
    pub feedback_store: FeedbackStore,
    pub dataset_store: DatasetStore,
    pub trace_converter: TraceConverter,
    pub auto_improvement_policy: Option<AutoImprovementPolicy>,
    pub auto_improvement_status: AutoImprovementStatus,
    pub model_registry: ModelRegistry,
}

impl TrainingRuntimeState {
    pub fn validate_invariants(&self) -> Result<(), MoeError> {
        if let Some(ref active_model_version) = self.model_registry.active_model_version
            && !self
                .model_registry
                .entries
                .iter()
                .any(|entry| entry.model_version == *active_model_version)
        {
            return Err(MoeError::PolicyRejected(format!(
                "training invariant failed: active model version {} is missing from registry",
                active_model_version
            )));
        }

        if let Some(latest_model_version) = self.model_registry.latest_model_version()
            && self.model_registry.next_model_version <= latest_model_version
        {
            return Err(MoeError::PolicyRejected(format!(
                "training invariant failed: next model version {} is not above latest {}",
                self.model_registry.next_model_version, latest_model_version
            )));
        }

        if let Some(ref last_bundle_checksum) = self.auto_improvement_status.last_bundle_checksum
            && self.auto_improvement_status.global_counters.runs_total == 0
        {
            return Err(MoeError::PolicyRejected(format!(
                "training invariant failed: last bundle checksum ({}) set while runs_total is zero",
                last_bundle_checksum
            )));
        }

        Ok(())
    }
}
