use crate::orchestrator::MoePipelineBuilder;

#[test]
fn builder_default_constructs_pipeline() {
    let pipeline = MoePipelineBuilder::default().build();
    assert_eq!(pipeline.registry().count(), 0);
}
