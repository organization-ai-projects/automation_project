//! projects/products/unstable/neurosymbolic_moe/backend/src/main.rs
mod aggregator;
mod app;
mod apps;
mod buffer_manager;
mod dataset_engine;
mod echo_expert;
mod evaluation_engine;
mod expert_registry;
mod feedback_engine;
mod memory_engine;
mod moe_core;
mod orchestrator;
mod policy_guard;
mod retrieval_engine;
mod router;
mod trace_logger;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
