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
        if let Some(active_version) = self.model_registry.active_version
            && !self
                .model_registry
                .entries
                .iter()
                .any(|entry| entry.version == active_version)
        {
            return Err(MoeError::PolicyRejected(format!(
                "training invariant failed: active model version {} is missing from registry",
                active_version
            )));
        }

        if let Some(latest_version) = self.model_registry.latest_version()
            && self.model_registry.next_version <= latest_version
        {
            return Err(MoeError::PolicyRejected(format!(
                "training invariant failed: next model version {} is not above latest {}",
                self.model_registry.next_version, latest_version
            )));
        }

        if self.auto_improvement_status.last_bundle_checksum.is_some()
            && self.auto_improvement_status.runs_total == 0
        {
            return Err(MoeError::PolicyRejected(
                "training invariant failed: last bundle checksum set while runs_total is zero"
                    .to_string(),
            ));
        }

        Ok(())
    }
}
