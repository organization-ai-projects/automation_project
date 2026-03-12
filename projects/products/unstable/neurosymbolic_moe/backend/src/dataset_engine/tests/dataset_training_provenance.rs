use crate::dataset_engine::DatasetTrainingProvenance;

#[test]
fn default_training_provenance_is_empty() {
    let provenance = DatasetTrainingProvenance::default();
    assert!(provenance.generator.is_empty());
    assert_eq!(provenance.governance_state_version, 0);
    assert!(provenance.governance_state_checksum.is_empty());
    assert!(provenance.runtime_bundle_checksum.is_empty());
    assert_eq!(provenance.dataset_entry_count, 0);
}
