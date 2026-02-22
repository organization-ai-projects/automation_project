use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatePath(String);

impl StatePath {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let v = value.into();
        if v.trim().is_empty() {
            None
        } else {
            Some(Self(v))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<Path> for StatePath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointPath(String);

impl CheckpointPath {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let v = value.into();
        if v.trim().is_empty() {
            None
        } else {
            Some(Self(v))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<Path> for CheckpointPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}
