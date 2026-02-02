// projects/libraries/identity/src/user_id.rs
use common::common_id::CommonID;
use common::custom_uuid::Id128;
use protocol::ProtocolId;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::IdentityError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(ProtocolId);

impl UserId {
    /// Creates a validated UserId
    pub fn new(id: ProtocolId) -> Result<Self, IdentityError> {
        if !CommonID::is_valid(id.as_inner()) {
            return Err(IdentityError::InvalidUserIdValue);
        }
        Ok(Self(id))
    }

    /// Returns the identifier as ProtocolId
    pub fn value(&self) -> ProtocolId {
        self.0
    }
}

// TryFrom for safe conversion from u64
impl TryFrom<u64> for UserId {
    type Error = IdentityError;

    fn try_from(id: u64) -> Result<Self, Self::Error> {
        Self::new(ProtocolId::new(Id128::new(id as u16, None, None)))
    }
}

// Implementation of the FromStr trait for UserId
impl FromStr for UserId {
    type Err = IdentityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = ProtocolId::from_str(s.trim()).map_err(|_| IdentityError::InvalidUserIdFormat)?;
        Self::new(id)
    }
}

// Conversion to String
impl From<UserId> for String {
    fn from(user_id: UserId) -> Self {
        user_id.0.to_string()
    }
}

// TryFrom for safe conversion from Id128
impl TryFrom<Id128> for UserId {
    type Error = IdentityError;

    fn try_from(id: Id128) -> Result<Self, Self::Error> {
        Self::new(ProtocolId::new(id))
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
