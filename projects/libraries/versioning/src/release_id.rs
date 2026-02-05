// projects/libraries/versioning/src/release_id.rs

use serde::{Deserialize, Serialize};
use std::fmt;

/// Custom version identifier using a three-tier numbering scheme
/// Tier 1: Major changes (breaking compatibility)
/// Tier 2: Feature additions (backward compatible)  
/// Tier 3: Corrections and refinements
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ReleaseId {
    tier_one: u32,
    tier_two: u32,
    tier_three: u32,
}

impl ReleaseId {
    /// Construct a new release identifier
    pub fn build(tier_one: u32, tier_two: u32, tier_three: u32) -> Self {
        Self {
            tier_one,
            tier_two,
            tier_three,
        }
    }

    /// Create initial release marker
    pub fn initial() -> Self {
        Self::build(1, 0, 0)
    }

    /// Get the first tier value
    pub fn first_tier(&self) -> u32 {
        self.tier_one
    }

    /// Get the second tier value
    pub fn second_tier(&self) -> u32 {
        self.tier_two
    }

    /// Get the third tier value
    pub fn third_tier(&self) -> u32 {
        self.tier_three
    }

    /// Advance to next major tier (resets others)
    pub fn advance_major(&self) -> Self {
        Self::build(self.tier_one + 1, 0, 0)
    }

    /// Advance to next feature tier (resets corrections)
    pub fn advance_feature(&self) -> Self {
        Self::build(self.tier_one, self.tier_two + 1, 0)
    }

    /// Advance correction tier
    pub fn advance_correction(&self) -> Self {
        Self::build(self.tier_one, self.tier_two, self.tier_three + 1)
    }

    /// Parse from string format "X.Y.Z"
    pub fn parse_str(input: &str) -> Result<Self, ReleaseIdError> {
        let segments: Vec<&str> = input.split('.').collect();
        
        if segments.len() != 3 {
            return Err(ReleaseIdError::InvalidFormat);
        }

        let tier_one = segments[0]
            .parse::<u32>()
            .map_err(|_| ReleaseIdError::InvalidNumber)?;
        let tier_two = segments[1]
            .parse::<u32>()
            .map_err(|_| ReleaseIdError::InvalidNumber)?;
        let tier_three = segments[2]
            .parse::<u32>()
            .map_err(|_| ReleaseIdError::InvalidNumber)?;

        Ok(Self::build(tier_one, tier_two, tier_three))
    }

    /// Check if this represents a breaking change from another version
    pub fn breaks_compatibility_with(&self, other: &Self) -> bool {
        self.tier_one != other.tier_one
    }
}

impl fmt::Display for ReleaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.tier_one, self.tier_two, self.tier_three)
    }
}

impl std::str::FromStr for ReleaseId {
    type Err = ReleaseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReleaseIdError {
    #[error("Invalid format - expected X.Y.Z")]
    InvalidFormat,
    #[error("Invalid number in version")]
    InvalidNumber,
}
