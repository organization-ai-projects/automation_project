use crate::buffer_manager::BufferManager;
use crate::orchestrator::{
    AutoImprovementStatus, GovernancePersistenceBundle, GovernanceState, ModelRegistry,
    RuntimeBundleComponents,
};

#[test]
fn runtime_bundle_components_is_constructible() {
    let components = RuntimeBundleComponents {
        governance: GovernancePersistenceBundle {
            state: GovernanceState::from_components(0, None, None, None),
            audit_entries: Vec::new(),
            snapshots: Vec::new(),
        },
        short_term_memory_entries: Vec::new(),
        long_term_memory_entries: Vec::new(),
        buffer_manager: BufferManager::new(8),
        dataset_entries: Vec::new(),
        dataset_corrections: std::collections::HashMap::new(),
        auto_improvement_policy: None,
        auto_improvement_status: AutoImprovementStatus::default(),
        model_registry: ModelRegistry::default(),
        trainer_trigger_events: Vec::new(),
    };
    assert_eq!(components.short_term_memory_entries.len(), 0);
    assert_eq!(components.long_term_memory_entries.len(), 0);
    assert_eq!(components.dataset_entries.len(), 0);
}
