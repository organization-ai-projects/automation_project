use crate::orchestrator::ModelRegistry;

#[test]
fn model_registry_registers_and_promotes_versions() {
    let mut registry = ModelRegistry::default();
    let v1 = registry.register_candidate("sum-1".to_string(), 10, 8, 2, 1);
    let v2 = registry.register_candidate("sum-2".to_string(), 20, 16, 4, 2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(registry.entry_count(), 2);
    assert_eq!(registry.latest_version(), Some(2));
    assert!(registry.promote(2));
    assert_eq!(registry.active_version, Some(2));
}
