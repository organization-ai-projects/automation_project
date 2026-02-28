use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TitleId(pub String);

impl fmt::Display for TitleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for TitleId {
    fn from(s: String) -> Self {
        TitleId(s)
    }
}

impl From<&str> for TitleId {
    fn from(s: &str) -> Self {
        TitleId(s.to_string())
    }
}
