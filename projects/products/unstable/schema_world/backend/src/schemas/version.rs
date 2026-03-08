use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    pub value: u32,
}

impl Default for Version {
    fn default() -> Self {
        Self { value: 1 }
    }
}
