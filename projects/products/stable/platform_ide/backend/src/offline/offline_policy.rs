// projects/products/stable/platform_ide/backend/src/offline/offline_policy.rs
use serde::{Deserialize, Serialize};

use crate::errors::IdeError;

/// Platform-approved offline mode policy.
///
/// Offline mode is **disabled by default** unless the platform explicitly
/// signals admin approval via its policy API. The IDE only exposes
/// offline-related controls when `allowed` is `true`.
///
/// No full offline implementation is required for MVP; this type is
/// policy plumbing only.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OfflinePolicy {
    /// Whether offline mode has been admin-approved by the platform.
    pub allowed: bool,
    /// An optional message to display if offline controls are available.
    pub notice: Option<String>,
}

impl OfflinePolicy {
    /// Returns the default (offline-disabled) policy.
    pub fn disabled() -> Self {
        Self::default()
    }

    /// Returns `true` if offline mode is permitted.
    pub fn is_allowed(&self) -> bool {
        self.allowed
    }

    /// Asserts that offline mode is permitted, returning an error otherwise.
    pub fn require_allowed(&self) -> Result<(), IdeError> {
        if self.allowed {
            Ok(())
        } else {
            Err(IdeError::OfflineNotPermitted)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_disabled() {
        let policy = OfflinePolicy::default();
        assert!(!policy.is_allowed());
        assert!(policy.require_allowed().is_err());
    }

    #[test]
    fn disabled_helper_matches_default() {
        let policy = OfflinePolicy::disabled();
        assert!(!policy.is_allowed());
    }

    #[test]
    fn allowed_policy_permits() {
        let policy = OfflinePolicy {
            allowed: true,
            notice: Some("Admin has approved offline access.".to_string()),
        };
        assert!(policy.is_allowed());
        assert!(policy.require_allowed().is_ok());
    }
}
