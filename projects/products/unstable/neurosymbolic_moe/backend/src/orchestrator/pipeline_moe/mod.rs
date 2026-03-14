//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/mod.rs
mod execution;
mod governance_runtime;
mod moe_pipeline;
mod observability;
mod persistence;

use crate::orchestrator::pipeline_moe::moe_pipeline::{
    MAX_GOVERNANCE_BUNDLE_JSON_BYTES, MAX_GOVERNANCE_STATE_JSON_BYTES,
    MAX_RUNTIME_BUNDLE_JSON_BYTES, MAX_RUNTIME_BUNDLE_SESSION_COUNT,
    MAX_RUNTIME_BUNDLE_SESSION_VALUES_TOTAL, MAX_RUNTIME_BUNDLE_TOTAL_MEMORY_ENTRIES,
    MAX_RUNTIME_BUNDLE_WORKING_ENTRIES,
};
pub use moe_pipeline::MoePipeline;
