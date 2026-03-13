//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/tests/concurrency_checks.rs

use crate::apps::DynError;
use crate::apps::run_concurrent_pipeline_checks;

#[test]
fn concurrency_checks_exports_are_wired() {
    let check_fn: fn() -> Result<(), DynError> = run_concurrent_pipeline_checks;
    let _ = check_fn;
}
