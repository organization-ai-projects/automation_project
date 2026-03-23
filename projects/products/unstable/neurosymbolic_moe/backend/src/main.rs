//! projects/products/unstable/neurosymbolic_moe/backend/src/main.rs
mod aggregator;
mod app;
mod apps;
mod buffer_manager;
mod dataset_engine;
mod delivery_stats;
mod echo_expert;
mod evaluations;
mod expert_registries;
mod feedback_engine;
mod fingerprint;
mod global_counters;
mod memory_engine;
mod moe_core;
mod orchestrator;
mod policies_guard;
mod retrieval_engine;
mod router;
mod skip_counters;
mod specialized_expert;
mod trace_logging;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
