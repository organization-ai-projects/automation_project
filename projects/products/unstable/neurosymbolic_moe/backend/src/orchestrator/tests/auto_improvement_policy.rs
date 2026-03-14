use crate::dataset_engine::DatasetTrainingBuildOptions;
use crate::orchestrator::AutoImprovementPolicy;

#[test]
fn auto_improvement_policy_defaults_are_safe() {
    let policy = AutoImprovementPolicy::default();
    assert!(policy.min_dataset_entries >= 1);
    assert!(policy.min_success_ratio >= 0.0);
    assert!(policy.min_success_ratio <= 1.0);
}

#[test]
fn auto_improvement_policy_builder_methods_override_values() {
    let options = DatasetTrainingBuildOptions {
        generated_at: 12,
        validation_ratio: 0.25,
        min_score: Some(0.55),
        include_failure_entries: true,
        include_partial_entries: false,
        include_unknown_entries: false,
        require_correction_for_failure: false,
        split_seed: 99,
    };
    let policy = AutoImprovementPolicy::default()
        .with_min_dataset_entries(8)
        .with_min_success_ratio(0.8)
        .with_min_average_score(Some(0.7))
        .with_training_build_options(options.clone());
    assert_eq!(policy.min_dataset_entries, 8);
    assert!((policy.min_success_ratio - 0.8).abs() < f64::EPSILON);
    assert_eq!(policy.min_average_score, Some(0.7));
    assert_eq!(
        policy.training_build_options.generated_at,
        options.generated_at
    );
    assert_eq!(
        policy.training_build_options.validation_ratio,
        options.validation_ratio
    );
}
