//! projects/products/unstable/autonomous_dev_ai/src/models/mod.rs
mod manager;

pub(crate) use manager::{infer_decision_action, infer_failure_kind, infer_failure_tool};
