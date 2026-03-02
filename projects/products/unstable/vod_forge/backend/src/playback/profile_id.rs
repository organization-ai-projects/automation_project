use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ProfileId(pub String);

impl fmt::Display for ProfileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ProfileId {
    fn from(s: String) -> Self {
        ProfileId(s)
    }
}

impl From<&str> for ProfileId {
    fn from(s: &str) -> Self {
        ProfileId(s.to_string())
    }
}
