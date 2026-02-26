use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandLineSpec {
    pub command: String,
    pub args: Vec<String>,
}
