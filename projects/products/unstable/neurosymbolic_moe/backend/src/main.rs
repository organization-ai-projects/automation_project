//! projects/products/unstable/neurosymbolic_moe/backend/src/main.rs
mod aggregator;
mod app;
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
#[cfg(test)]
mod tests;
mod trace_logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
