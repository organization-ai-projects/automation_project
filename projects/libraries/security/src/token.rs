// projects/libraries/security/src/token.rs
use crate::role::Role;
use crate::token_error::TokenError;
use common::common_id::is_valid_id;
use common_time::timestamp_utils::current_timestamp_ms;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub value: String,
    pub user_id: String,
    pub role: Role,

    /// Milliseconds since Unix epoch (UTC)
    pub issued_at_ms: u64,
    /// Milliseconds since Unix epoch (UTC)
    pub expires_at_ms: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl Token {
    /// Creates a token lasting `duration_ms` milliseconds.
    pub fn new(user_id: String, role: Role, duration_ms: u64) -> Result<Self, TokenError> {
        if duration_ms == 0 {
            return Err(TokenError::InvalidDuration);
        }

        // Validate user_id according to your existing common::utils::is_valid_id contract.
        let numeric_id = user_id
            .parse::<u64>()
            .map_err(|_| TokenError::InvalidUserIdFormat)?;

        if !is_valid_id(numeric_id) {
            return Err(TokenError::InvalidUserIdValue);
        }

        let now = current_timestamp_ms();
        let expires = now
            .checked_add(duration_ms)
            .ok_or(TokenError::TimestampOverflow)?;

        Ok(Self {
            // Proper UUIDv7 (time-ordered, includes randomness)
            value: Uuid::now_v7().to_string(),
            user_id,
            role,
            issued_at_ms: now,
            expires_at_ms: expires,
            session_id: None,
        })
    }

    /// Creates a token with an attached session id.
    pub fn new_with_session(
        user_id: String,
        role: Role,
        duration_ms: u64,
        session_id: String,
    ) -> Result<Self, TokenError> {
        if session_id.trim().is_empty() {
            return Err(TokenError::InvalidSessionId);
        }

        let mut token = Self::new(user_id, role, duration_ms)?;
        token.session_id = Some(session_id);
        Ok(token)
    }

    pub fn is_expired(&self) -> bool {
        current_timestamp_ms() >= self.expires_at_ms
    }

    /// Remaining time in milliseconds until expiry.
    pub fn time_until_expiry_ms(&self) -> u64 {
        let now = current_timestamp_ms();
        self.expires_at_ms.saturating_sub(now)
    }

    /// Extends expiry by `additional_ms`.
    /// If already expired, it renews from "now".
    pub fn renew(&mut self, additional_ms: u64) -> Result<(), TokenError> {
        if additional_ms == 0 {
            return Err(TokenError::InvalidDuration);
        }

        let now = current_timestamp_ms();

        let base = if now >= self.expires_at_ms {
            now
        } else {
            self.expires_at_ms
        };

        self.expires_at_ms = base
            .checked_add(additional_ms)
            .ok_or(TokenError::TimestampOverflow)?;

        Ok(())
    }

    /// Age of the token in milliseconds.
    pub fn age_ms(&self) -> u64 {
        let now = current_timestamp_ms();
        now.saturating_sub(self.issued_at_ms)
    }

    /// Optional: quick structural validation.
    pub fn validate(&self) -> bool {
        // value must be non-empty
        if self.value.trim().is_empty() {
            return false;
        }

        // user_id must still be valid
        let numeric_id = match self.user_id.parse::<u64>() {
            Ok(v) => v,
            Err(_) => return false,
        };
        if !is_valid_id(numeric_id) {
            return false;
        }

        // timestamps must be sane
        self.expires_at_ms >= self.issued_at_ms
    }
}

/// Simple helper: "valid and not expired".
pub fn validate_token(token: &Token) -> bool {
    token.validate() && !token.is_expired()
}

/// “Dev helper” (je te conseille de le garder en test ou tooling)
#[cfg(test)]
pub fn generate_token_for_tests(duration_ms: u64) -> Token {
    Token::new("1".to_string(), Role::User, duration_ms).expect("test token")
}

/// Placeholder for revocation logic (blacklist / store / cache, etc.)
pub fn revoke_token(_token: &Token) {
    // Intentionally empty: implement via a revocation store (in-memory/redis/db) if needed.
}
