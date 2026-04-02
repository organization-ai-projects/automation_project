//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/tests/concurrency_checks.rs

use crate::apps::DynError;
use crate::apps::{run_concurrent_pipeline_checks, run_concurrent_pipeline_checks_with_report};
use crate::orchestrator::ConcurrentOperationalReport;

#[test]
fn concurrency_checks_exports_are_wired() {
    let check_fn: fn() -> Result<(), DynError> = run_concurrent_pipeline_checks;
    let check_with_report_fn: fn() -> Result<ConcurrentOperationalReport, DynError> =
        run_concurrent_pipeline_checks_with_report;
    let _ = check_fn;
    let _ = check_with_report_fn;
}
