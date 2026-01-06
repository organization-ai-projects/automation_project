use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub value: String,
    pub expires_at: u64,
}

pub fn validate_token(token: &Token) -> bool {
    token.expires_at > Utc::now().timestamp() as u64
}

pub fn generate_token(duration_secs: u64) -> Token {
    Token {
        value: Uuid::new_v4().to_string(),
        expires_at: (Utc::now() + Duration::seconds(duration_secs as i64)).timestamp() as u64,
    }
}

pub fn revoke_token(_token: &Token) {
    // Placeholder for token revocation logic
}
