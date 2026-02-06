pub mod repo;
pub mod gh;
pub mod ci;

pub use repo::{RepoAdapter, RepoContext};
pub use gh::{GhAdapter, GhContext};
pub use ci::{CiAdapter, CiContext};
