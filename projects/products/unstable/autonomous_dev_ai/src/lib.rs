// projects/products/unstable/autonomous_dev_ai/src/lib.rs

// Core modules
pub mod config;
pub mod error;
pub mod memory;
pub mod objectives;
pub mod state;

// Symbolic layer (authoritative)
pub mod symbolic;

// Neural layer (advisory)
pub mod neural;

// Tools
pub mod tools;

// Agent lifecycle
pub mod agent;
pub mod lifecycle;

// Audit
pub mod audit;

// Re-exports
pub use agent::AutonomousAgent;
pub use config::{AgentConfig, load_config, save_ron};
pub use error::AgentError;
pub use state::AgentState;
