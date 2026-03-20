use serde::{Deserialize, Serialize};

use crate::orchestrator::Version;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionDeltaDirection {
    Upgrade,
    Same,
    Regression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionDeltaLevel {
    None,
    Patch,
    Minor,
    Major,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionDelta {
    pub direction: VersionDeltaDirection,
    /// Semver amplitude independent of direction.
    pub level: VersionDeltaLevel,
    pub major_delta: u32,
    pub minor_delta: u32,
    pub patch_delta: u32,
    pub major_changed: bool,
    pub minor_changed: bool,
    pub patch_changed: bool,
}

impl VersionDelta {
    pub fn between(source: &Version, target: &Version) -> Self {
        let direction = match target.cmp(source) {
            std::cmp::Ordering::Greater => VersionDeltaDirection::Upgrade,
            std::cmp::Ordering::Equal => VersionDeltaDirection::Same,
            std::cmp::Ordering::Less => VersionDeltaDirection::Regression,
        };
        let major_delta = target.major.abs_diff(source.major);
        let minor_delta = target.minor.abs_diff(source.minor);
        let patch_delta = target.patch.abs_diff(source.patch);
        let major_changed = major_delta > 0;
        let minor_changed = minor_delta > 0;
        let patch_changed = patch_delta > 0;
        let level = if major_changed {
            VersionDeltaLevel::Major
        } else if minor_changed {
            VersionDeltaLevel::Minor
        } else if patch_changed {
            VersionDeltaLevel::Patch
        } else {
            VersionDeltaLevel::None
        };

        Self {
            direction,
            level,
            major_delta,
            minor_delta,
            patch_delta,
            major_changed,
            minor_changed,
            patch_changed,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.direction == VersionDeltaDirection::Same
    }

    pub fn is_regression(&self) -> bool {
        self.direction == VersionDeltaDirection::Regression
    }

    pub fn exceeds_limit(&self, limit: &Version) -> bool {
        (self.major_delta, self.minor_delta, self.patch_delta)
            > (limit.major, limit.minor, limit.patch)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStateDiff {
    pub source_version: Version,
    pub target_version: Version,
    pub version_delta: VersionDelta,
    pub schema_version_changed: bool,
    pub checksum_changed: bool,
    pub policy_changed: bool,
    pub baseline_changed: bool,
    pub report_changed: bool,
    pub has_drift: bool,
}
