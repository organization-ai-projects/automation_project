// projects/products/unstable/platform_versioning/backend/src/ids/tree_id.rs
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::PvError;
use crate::ids::ObjectId;

/// Content address of a tree (directory snapshot).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TreeId(ObjectId);

impl TreeId {
    /// Creates a `TreeId` from a raw 32-byte digest.
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        Self(ObjectId::from_bytes(bytes))
    }

    /// Returns the underlying [`ObjectId`].
    pub fn as_object_id(&self) -> &ObjectId {
        &self.0
    }

    /// Returns the hex string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for TreeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl FromStr for TreeId {
    type Err = PvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl From<ObjectId> for TreeId {
    fn from(id: ObjectId) -> Self {
        Self(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let raw = [0x02u8; 32];
        let id = TreeId::from_bytes(&raw);
        let parsed: TreeId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }
}
