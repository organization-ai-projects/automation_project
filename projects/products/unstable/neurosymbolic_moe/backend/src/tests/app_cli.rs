//! projects/products/unstable/neurosymbolic_moe/backend/src/tests/app_cli.rs

use crate::app::parse_run_options;
use crate::moe_core::TaskType;

#[test]
fn parse_run_options_defaults_to_code_generation() {
    let parsed = parse_run_options(&["ship".to_string(), "feature".to_string()]).unwrap();

    assert!(matches!(parsed.task_type, TaskType::CodeGeneration));
    assert_eq!(parsed.input, "ship feature");
    assert!(parsed.bootstrap_dataset_bundle_path.is_none());
}

#[test]
fn parse_run_options_supports_explicit_task_type_and_bootstrap_path() {
    let parsed = parse_run_options(&[
        "--task-type".to_string(),
        "validation".to_string(),
        "--bootstrap-dataset-bundle-json".to_string(),
        "bundle.json".to_string(),
        "review".to_string(),
        "this".to_string(),
    ])
    .unwrap();

    assert!(matches!(parsed.task_type, TaskType::Validation));
    assert_eq!(parsed.input, "review this");
    assert_eq!(
        parsed.bootstrap_dataset_bundle_path.as_deref(),
        Some("bundle.json")
    );
}
