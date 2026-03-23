//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/tests/moe_pipeline.rs
use crate::orchestrator::MoePipelineBuilder;

#[test]
fn moe_pipeline_module_exposes_pipeline_type() {
    let pipeline = MoePipelineBuilder::new().build();
    assert_eq!(pipeline.trainer_trigger_events_pending(), 0);
}
