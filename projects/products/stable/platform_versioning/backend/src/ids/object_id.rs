// projects/products/stable/platform_versioning/backend/src/ids/object_id.rs
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::PvError;

/// A 32-byte SHA-256 content address encoded as a 64-character hex string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ObjectId(String);

impl ObjectId {
    /// The byte length of a raw SHA-256 digest.
    pub const RAW_LEN: usize = 32;
    /// The hex-encoded length of a SHA-256 digest.
    pub const HEX_LEN: usize = 64;

    /// Creates an `ObjectId` from a raw 32-byte SHA-256 digest.
    pub fn from_bytes(bytes: &[u8; Self::RAW_LEN]) -> Self {
        Self(hex::encode(bytes))
    }

    /// Returns the hex string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the raw 32-byte digest.
    ///
    /// # Panics
    /// Panics if the internal invariant is broken and the hex string is invalid.
    pub fn to_bytes(&self) -> [u8; Self::RAW_LEN] {
        let mut out = [0u8; Self::RAW_LEN];
        // Invariant: ObjectId always stores 64 valid hex chars. If this fails,
        // fail closed in all build modes rather than returning corrupted bytes.
        assert!(
            hex::decode_to_slice(&self.0, &mut out).is_ok(),
            "ObjectId contains invalid hex — construction-time invariant violated"
        );
        out
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for ObjectId {
    type Err = PvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != Self::HEX_LEN {
            return Err(PvError::InvalidId(format!(
                "ObjectId must be {} hex chars, got {}",
                Self::HEX_LEN,
                s.len()
            )));
        }
        if !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(PvError::InvalidId(
                "ObjectId must contain only hex characters".to_string(),
            ));
        }
        Ok(Self(s.to_lowercase()))
    }
}
