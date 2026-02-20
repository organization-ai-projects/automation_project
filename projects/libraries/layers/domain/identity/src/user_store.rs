// projects/libraries/layers/domain/identity/src/user_store.rs
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

use rand::RngCore;
use security_core::{PasswordError, Role, password};

use crate::{IdentityError, User, UserId};

/// In-memory user store with secure password hashing
#[derive(Clone)]
pub struct UserStore {
    users: Arc<RwLock<HashMap<UserId, User>>>,
}

impl UserStore {
    /// Create a new empty user store
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn fallback_password_hash() -> Result<&'static str, IdentityError> {
        static FALLBACK_HASH: OnceLock<String> = OnceLock::new();
        if let Some(hash) = FALLBACK_HASH.get() {
            return Ok(hash.as_str());
        }

        let hash = password::hash_password(&random_fallback_secret())?;
        let _ = FALLBACK_HASH.set(hash);

        Ok(FALLBACK_HASH
            .get()
            .expect("fallback password hash should be initialized")
            .as_str())
    }

    /// Add a user to the store
    pub async fn add_user(
        &self,
        user_id: UserId,
        password: &str,
        role: Role,
    ) -> Result<(), IdentityError> {
        if password.trim().is_empty() {
            return Err(IdentityError::EmptyPassword);
        }

        let password_hash = password::hash_password(password)?;
        let user = User {
            user_id: user_id.clone(),
            password_hash,
            role,
        };

        let mut users = self.users.write().await;
        users.insert(user_id, user);
        Ok(())
    }

    /// Authenticate a user and return their role if successful
    pub async fn authenticate(
        &self,
        user_id: &UserId,
        password: &str,
    ) -> Result<Role, IdentityError> {
        let (password_hash, role) = {
            let users = self.users.read().await;
            match users.get(user_id) {
                Some(user) => (Some(user.password_hash.clone()), Some(user.role)),
                None => (None, None),
            }
        };

        if let (Some(password_hash), Some(role)) = (password_hash, role) {
            if password::verify_password(password, &password_hash)? {
                Ok(role)
            } else {
                Err(IdentityError::InvalidCredentials)
            }
        } else {
            let fallback_hash = Self::fallback_password_hash()?;
            let _ = password::verify_password(password, fallback_hash);
            Err(IdentityError::InvalidCredentials)
        }
    }

    /// Check if a user exists
    pub async fn user_exists(&self, user_id: &UserId) -> bool {
        let users = self.users.read().await;
        users.contains_key(user_id)
    }

    /// Get user role
    pub async fn get_user_role(&self, user_id: &UserId) -> Option<Role> {
        let users = self.users.read().await;
        users.get(user_id).map(|u| u.role)
    }

    /// Count total users
    pub async fn user_count(&self) -> usize {
        let users = self.users.read().await;
        users.len()
    }
}

fn random_fallback_secret() -> String {
    let mut bytes = [0u8; 32];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut bytes);
    bytes_to_hex(&bytes)
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(nibble_to_hex(byte >> 4));
        out.push(nibble_to_hex(byte & 0x0f));
    }
    out
}

fn nibble_to_hex(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => '0',
    }
}

impl Default for UserStore {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PasswordError> for IdentityError {
    fn from(err: PasswordError) -> Self {
        match err {
            PasswordError::HashError(message) => IdentityError::PasswordHashError(message),
        }
    }
}
