//! projects/products/unstable/neurosymbolic_moe/backend/src/apps/mod.rs
mod concurrency_checks;
mod dyn_error;
mod impl_check;
mod runtime_checks;
#[cfg(test)]
mod tests;

pub(crate) use concurrency_checks::{
    run_concurrent_pipeline_checks, run_concurrent_pipeline_checks_with_report,
};
pub(crate) use dyn_error::DynError;
pub(crate) use impl_check::cmd_impl_check;
pub(crate) use runtime_checks::{
    run_runtime_persistence_checks, run_runtime_persistence_checks_with_report,
    run_training_and_cas_checks,
};
