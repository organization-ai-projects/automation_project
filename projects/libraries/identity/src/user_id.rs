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
