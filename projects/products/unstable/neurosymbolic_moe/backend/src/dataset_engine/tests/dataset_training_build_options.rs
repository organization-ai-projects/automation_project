use crate::dataset_engine::DatasetTrainingBuildOptions;

#[test]
fn default_training_build_options_are_conservative() {
    let options = DatasetTrainingBuildOptions::default();
    assert!((options.validation_ratio - 0.1).abs() < f64::EPSILON);
    assert_eq!(options.min_score, None);
    assert!(options.include_failure_entries);
    assert!(!options.include_partial_entries);
    assert!(!options.include_unknown_entries);
    assert!(options.require_correction_for_failure);
    assert_eq!(options.split_seed, 0);
    assert_eq!(options.generated_at, 0);
}
