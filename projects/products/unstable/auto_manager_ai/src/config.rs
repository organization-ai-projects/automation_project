// projects/products/unstable/auto_manager_ai/src/config.rs

use std::path::PathBuf;
use crate::domain::Policy;

/// Configuration for the automation manager
#[derive(Debug, Clone)]
pub struct Config {
    pub repo_path: PathBuf,
    pub output_dir: PathBuf,
    pub policy: Policy,
}

impl Config {
    /// Create a new configuration
    pub fn new(repo_path: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            repo_path,
            output_dir,
            policy: Policy::default(),
        }
    }
}
