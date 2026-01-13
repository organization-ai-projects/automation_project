// projects/products/code_agent_sandbox/src/agents/mod.rs
pub mod agent_driver;
pub mod agent_outcome;
pub mod agent_request;
pub mod defaults;

pub use agent_driver::{
    build_context_snippet, default_score_cfg, detect_single_target_file,
    run_agent_with_orchestrator, run_and_record, truncate,
};
pub use agent_outcome::AgentOutcome;
pub use agent_request::AgentRequest;
pub use defaults::default_max_iters;
