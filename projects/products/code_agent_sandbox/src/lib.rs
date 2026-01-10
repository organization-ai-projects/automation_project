// projects/products/code_agent_sandbox/src/lib.rs
pub mod actions;
pub mod fs;
pub mod journal;
pub mod policy;
pub mod runner;
pub mod score;

pub use actions::{Action, ActionResult};
pub use fs::SandboxFs;
pub use journal::Journal;
pub use policy::Policy;
pub use runner::CommandRunner;
pub use score::{ScoreConfig, ScoreSummary};
