// projects/libraries/identity/src/user_id.rs
use common::common_id::CommonID;
use common::custom_uuid::Id128;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::IdentityError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(Id128);

impl UserId {
    /// Creates a validated UserId
    pub fn new(id: Id128) -> Result<Self, IdentityError> {
        if !CommonID::is_valid(id) {
            return Err(IdentityError::InvalidUserIdValue);
        }
        Ok(Self(id))
    }

    /// Returns the identifier as Id128
    pub fn value(&self) -> Id128 {
        self.0
    }
}

// TryFrom for safe conversion from u64
impl TryFrom<u64> for UserId {
    type Error = IdentityError;

    fn try_from(id: u64) -> Result<Self, Self::Error> {
        Self::new(Id128::new(id as u16, None, None))
    }
}

// Implementation of the FromStr trait for UserId
impl FromStr for UserId {
    type Err = IdentityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .trim()
            .parse::<u128>()
            .map_err(|_| IdentityError::InvalidUserIdFormat)?;
        let id128 = Id128::from_bytes_unchecked(id.to_be_bytes());
        Self::new(id128)
    }
}

// Conversion to String
impl From<UserId> for String {
    fn from(user_id: UserId) -> Self {
        user_id.0.to_string()
    }
}

// Implementation of From<Id128> for UserId
impl From<Id128> for UserId {
    fn from(id: Id128) -> Self {
        UserId(id)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Utility function to validate a user_id string
pub fn validate_user_id(user_id: &str) -> bool {
    user_id.parse::<UserId>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_new_valid() {
        let id = Id128::from_bytes_unchecked([1u8; 16]);
        let user_id = UserId::new(id).expect("user id");
        assert_eq!(user_id.value(), id);
    }

    #[test]
    fn test_user_id_new_invalid() {
        let invalid_id = Id128::from_bytes_unchecked([0u8; 16]);
        assert!(UserId::new(invalid_id).is_err());
    }

    #[test]
    fn test_user_id_equality() {
        let id1 = Id128::from_bytes_unchecked([1u8; 16]);
        let id2 = Id128::from_bytes_unchecked([1u8; 16]);
        let id3 = Id128::from_bytes_unchecked([2u8; 16]);

        let user_id1 = UserId::new(id1).expect("user id1");
        let user_id2 = UserId::new(id2).expect("user id2");
        let user_id3 = UserId::new(id3).expect("user id3");

        assert_eq!(user_id1, user_id2);
        assert_ne!(user_id1, user_id3);
    }

    #[test]
    fn test_user_id_display() {
        let id = Id128::from_bytes_unchecked([42u8; 16]);
        let user_id = UserId::new(id).expect("user id");
        assert_eq!(format!("{}", user_id), id.to_string());
    }

    #[test]
    fn test_validate_user_id() {
        // Check that "0" is invalid
        let invalid_id = Id128::from_bytes_unchecked([0u8; 16]);
        assert!(!CommonID::is_valid(invalid_id));
    }

    #[test]
    fn test_user_id_new_with_id128() {
        // Test directly with Id128
        let valid_id = Id128::from_bytes_unchecked([1u8; 16]);
        let invalid_id = Id128::from_bytes_unchecked([0u8; 16]);

        assert!(CommonID::is_valid(valid_id));
        assert!(!CommonID::is_valid(invalid_id));
    }
}
