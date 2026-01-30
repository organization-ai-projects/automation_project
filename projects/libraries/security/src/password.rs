// projects/libraries/security/src/password.rs
use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::PasswordError;

fn argon2() -> Result<Argon2<'static>, PasswordError> {
    const ARGON2_MEMORY_COST_KIB: u32 = 19_456;
    const ARGON2_TIME_COST: u32 = 2;
    const ARGON2_PARALLELISM: u32 = 1;

    let params = Params::new(
        ARGON2_MEMORY_COST_KIB,
        ARGON2_TIME_COST,
        ARGON2_PARALLELISM,
        None,
    )
    .map_err(|e| PasswordError::HashError(e.to_string()))?;

    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

/// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2()?;

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| PasswordError::HashError(e.to_string()))
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| PasswordError::HashError(e.to_string()))?;

    let argon2 = argon2()?;
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
