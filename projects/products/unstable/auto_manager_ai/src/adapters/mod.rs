// projects/products/unstable/auto_manager_ai/src/adapters/mod.rs

pub mod ci_adapter;
pub mod ci_context;
pub mod error;
pub mod gh_adapter;
pub mod gh_context;
pub mod repo_adapter;
pub mod repo_context;

pub use ci_adapter::CiAdapter;
pub use ci_context::CiContext;
pub use gh_adapter::GhAdapter;
pub use gh_context::GhContext;
pub use repo_adapter::RepoAdapter;
pub use repo_context::RepoContext;
