// projects/libraries/security/src/token.rs
use crate::{TokenError, role::Role};
use common::custom_uuid::Id128;
use common_time::timestamp_utils;
use serde::{Deserialize, Serialize};

/// Verified token (internal struct convenient for the app).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub value: String,
    pub subject_id: Id128,
    pub role: Role,
    pub issued_at_ms: u64,
    pub expires_at_ms: u64,
    pub session_id: Option<String>,
}

impl Token {
    /// Checks if the token is expired
    pub fn is_expired(&self) -> bool {
        self.is_expired_with_grace(0)
    }

    /// Checks if the token is expired with an optional grace period
    pub fn is_expired_with_grace(&self, grace_ms: u64) -> bool {
        let now = timestamp_utils::current_timestamp_ms();
        self.expires_at_ms.saturating_add(grace_ms) <= now
    }

    /// Returns the remaining time until expiration in milliseconds
    pub fn time_until_expiry_ms(&self) -> i64 {
        let now = timestamp_utils::current_timestamp_ms() as i64;
        self.expires_at_ms as i64 - now
    }

    /// Returns the age of the token in milliseconds
    pub fn age_ms(&self) -> u64 {
        let now = timestamp_utils::current_timestamp_ms();
        now.saturating_sub(self.issued_at_ms)
    }

    /// Validates a token (structure + expiration)
    pub fn validate_token(&self) -> Result<(), TokenError> {
        let zero_id = Id128::from_bytes_unchecked([0u8; 16]);
        if self.subject_id == zero_id {
            return Err(TokenError::InvalidToken);
        }
        if self.is_expired() {
            return Err(TokenError::Expired);
        }
        Ok(())
    }
}
