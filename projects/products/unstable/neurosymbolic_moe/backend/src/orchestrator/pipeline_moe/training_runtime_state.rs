//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/training_runtime_state.rs
use crate::dataset_engine::{DatasetStore, TraceConverter};
use crate::feedback_engine::FeedbackStore;
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
