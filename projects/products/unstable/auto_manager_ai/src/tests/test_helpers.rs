// projects/products/unstable/auto_manager_ai/src/tests/test_helpers.rs

use crate::domain::{Action, ActionStatus, ActionTarget, RiskLevel};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

// Shared counter for unique test directory names
static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Create a unique temporary directory for tests
/// Uses an atomic counter and timestamp to ensure uniqueness even when tests run in parallel
pub(crate) fn create_unique_temp_dir(prefix: &str) -> PathBuf {
    let id = TEST_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    std::env::temp_dir().join(format!("{}_{}_{}", prefix, timestamp, id))
}

/// Build a test Action with common defaults
/// This reduces boilerplate in policy tests
pub(crate) fn build_test_action(action_type: &str, confidence: f64) -> Action {
    Action {
        id: "test_001".to_string(),
        action_type: action_type.to_string(),
        status: ActionStatus::Proposed,
        target: ActionTarget::Repo {
            reference: "test/repo".to_string(),
        },
        justification: "Test action".to_string(),
        risk_level: RiskLevel::Low,
        required_checks: vec![],
        confidence,
        evidence: vec![],
        depends_on: None,
        missing_inputs: None,
        dry_run: None,
    }
}
