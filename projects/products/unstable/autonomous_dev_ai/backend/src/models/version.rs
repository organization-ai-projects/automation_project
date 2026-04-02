//! projects/products/unstable/autonomous_dev_ai/src/models/version.rs
use serde::{Deserialize, Serialize};
use std::{
    cmp, fmt,
    hash::{Hash, Hasher},
};

/// A struct to represent a semantic version (MAJOR.MINOR.PATCH).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Version {
    pub(crate) major: u32,
    pub(crate) minor: u32,
    pub(crate) patch: u32,
}

impl Version {
    /// Creates a new version with the given major, minor, and patch.
    pub(crate) fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    /// Increments the major version, resetting minor and patch.
    pub(crate) fn increment_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
    }

    /// Increments the minor version, resetting patch.
    pub(crate) fn increment_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
    }

    /// Increments the patch version.
    pub(crate) fn increment_patch(&mut self) {
        self.patch += 1;
    }

    pub(crate) fn to_compact_u64(&self) -> u64 {
        ((self.major as u64) << 32) | ((self.minor as u64) << 16) | self.patch as u64
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::new(0, 0, 1)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        let major = (value >> 32) as u32;
        let minor = (value & 0xFFFF_FFFF) as u32;
        Version {
            major,
            minor,
            ..Version::default()
        }
    }
}

impl From<Version> for u64 {
    fn from(version: Version) -> Self {
        version.to_compact_u64()
    }
}

impl From<&Version> for u64 {
    fn from(version: &Version) -> Self {
        version.to_compact_u64()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Hash for Version {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.major.hash(state);
        self.minor.hash(state);
        self.patch.hash(state);
    }
}
