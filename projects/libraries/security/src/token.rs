use crate::role::Role;
use common::utils::is_valid_name;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::{Timestamp, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub value: String,
    pub user_id: String,
    pub role: Role,
    pub issued_at: u64,
    pub expires_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl Token {
    pub fn new(user_id: String, role: Role, duration_secs: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if !is_valid_name(&user_id) {
            panic!("Invalid user ID provided");
        }

        Self {
            value: Uuid::new_v7(Timestamp::from_unix(uuid::NoContext, 0, 0))
                .to_string(),
            user_id,
            role,
            issued_at: now,
            expires_at: now + duration_secs,
            session_id: None,
        }
    }

    pub fn new_with_session(
        user_id: String,
        role: Role,
        duration_secs: u64,
        session_id: String,
    ) -> Self {
        let mut token = Self::new(user_id, role, duration_secs);
        token.session_id = Some(session_id);
        token
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now >= self.expires_at
    }

    pub fn time_until_expiry(&self) -> Option<u64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if self.expires_at > now {
            Some(self.expires_at - now)
        } else {
            None
        }
    }

    pub fn renew(&mut self, additional_duration_secs: u64) {
        self.expires_at += additional_duration_secs;
    }

    pub fn age(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now > self.issued_at {
            now - self.issued_at
        } else {
            1 // Retourne un âge minimal de 1 si les horodatages sont identiques
        }
    }
}

pub fn validate_token(token: &Token) -> bool {
    !token.is_expired()
}

pub fn generate_token(duration_secs: u64) -> Token {
    Token::new("default_user".to_string(), Role::User, duration_secs)
}

pub fn revoke_token(_token: &Token) {
    // Placeholder for token revocation logic
}
