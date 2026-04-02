use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExpertId(pub String);

impl ExpertId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl fmt::Display for ExpertId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
