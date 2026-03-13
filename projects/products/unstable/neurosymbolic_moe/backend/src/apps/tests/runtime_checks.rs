//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/tests/runtime_checks.rs

use crate::apps::{run_runtime_persistence_checks, run_training_and_cas_checks};
use crate::{apps::DynError, orchestrator::MoePipeline};

#[test]
fn runtime_checks_exports_are_wired() {
    let runtime_check_fn: fn() -> Result<MoePipeline, DynError> = run_runtime_persistence_checks;
    let training_check_fn: fn(&mut MoePipeline) -> Result<(), DynError> =
        run_training_and_cas_checks;
    let _ = runtime_check_fn;
    let _ = training_check_fn;
}
