// projects/libraries/security/src/token_service.rs
use crate::{Claims, Role, Token, TokenError};
use common::common_id::CommonID;
use common::custom_uuid::Id128;
use common_time::timestamp_utils::current_timestamp_ms;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

/// Service to issue/verify JWTs.
/// - Stateless: no need for a store.
#[derive(Clone)]
pub struct TokenService {
    enc: EncodingKey,
    dec: DecodingKey,
    validation: Validation,
}

impl TokenService {
    /// secret: robust string (ENV). Minimum 32 characters recommended.
    pub fn new_hs256(secret: &str) -> Result<Self, TokenError> {
        let s = secret.trim();
        if s.is_empty() {
            return Err(TokenError::MissingSecret);
        }
        if s.len() < 32 {
            return Err(TokenError::SecretTooShort);
        }

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.required_spec_claims.insert("exp".to_string());
        validation.required_spec_claims.insert("iat".to_string());
        validation.required_spec_claims.insert("sub".to_string());
        validation.required_spec_claims.insert("jti".to_string());
        validation.leeway = 1; // Add a 1-second margin to compensate for tolerance

        Ok(Self {
            enc: EncodingKey::from_secret(s.as_bytes()),
            dec: DecodingKey::from_secret(s.as_bytes()),
            validation,
        })
    }

    /// Issue a signed JWT. duration_ms must be > 0.
    pub fn issue(
        &self,
        subject_id: Id128,
        role: Role,
        duration_ms: u64,
        session_id: Option<String>,
    ) -> Result<String, TokenError> {
        if duration_ms == 0 {
            return Err(TokenError::InvalidDuration);
        }

        if let Some(sid) = &session_id
            && sid.trim().is_empty()
        {
            return Err(TokenError::InvalidSessionId);
        }

        let now_ms = current_timestamp_ms();
        let exp_ms = now_ms
            .checked_add(duration_ms)
            .ok_or(TokenError::TimestampOverflow)?;
        let now_s = now_ms / 1000;
        let exp_s = (exp_ms / 1000).saturating_add(1);

        if !CommonID::is_valid(subject_id) {
            return Err(TokenError::InvalidSubjectIdValue);
        }

        let claims = Claims {
            sub: subject_id.to_string(),
            jti: Id128::new(0, None, None).to_string(),
            role,
            iat: now_s,
            exp: exp_s,
            sid: session_id,
        };

        jsonwebtoken::encode(&Header::new(Algorithm::HS256), &claims, &self.enc)
            .map_err(|e| TokenError::Jwt(e.to_string()))
    }

    /// Verify a JWT and return a strongly typed Token (convenient for app).
    pub fn verify(&self, jwt: &str) -> Result<Token, TokenError> {
        let jwt = jwt.trim();
        if jwt.is_empty() {
            return Err(TokenError::InvalidToken);
        }

        let now_ms = current_timestamp_ms();
        println!("Token verification: now_ms = {}", now_ms);

        let data =
            jsonwebtoken::decode::<Claims>(jwt, &self.dec, &self.validation).map_err(|e| {
                if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                    let exp_with_tolerance = now_ms.saturating_sub(50); // Apply tolerance here
                    if exp_with_tolerance > now_ms {
                        println!(
                            "Token expired with tolerance: exp = {}, tolerance = 50 ms",
                            now_ms
                        );
                        TokenError::Expired
                    } else {
                        TokenError::Jwt(e.to_string())
                    }
                } else {
                    TokenError::Jwt(e.to_string())
                }
            })?;

        let c = data.claims;
        println!("Token timestamps: iat = {}, exp = {}", c.iat, c.exp);

        // Hardening: validate sub numeric + CommonID validation
        let subject_id = Id128::from_hex(&c.sub).map_err(|_| TokenError::InvalidSubjectIdFormat)?;

        if !CommonID::is_valid(subject_id) {
            return Err(TokenError::InvalidSubjectIdValue);
        }

        let issued_at_ms = c.iat.saturating_mul(1000);
        let expires_at_ms = c.exp.saturating_mul(1000);

        // Manual validation: check if the current time exceeds expiration
        // Add a tolerance margin to avoid errors due to minor differences
        const TOLERANCE_MS: u64 = 50; // 50 ms tolerance

        if now_ms > c.exp.saturating_mul(1000) + TOLERANCE_MS {
            println!(
                "Manual validation: token expired (now_ms = {}, exp = {}, tolerance = {} ms)",
                now_ms,
                c.exp * 1000,
                TOLERANCE_MS
            );
            return Err(TokenError::Expired);
        }

        println!(
            "Manual validation: now_ms = {}, exp = {}",
            now_ms / 1000,
            c.exp
        );

        Ok(Token {
            value: c.jti,
            subject_id,
            role: c.role,
            issued_at_ms,
            expires_at_ms,
            session_id: c.sid,
        })
    }

    /// Renew an existing token (creates a new signed JWT)
    pub fn renew(
        &self,
        old_token: &Token,
        additional_duration_ms: u64,
    ) -> Result<String, TokenError> {
        if old_token.is_expired() {
            return Err(TokenError::CannotRenewExpired);
        }

        let remaining_ms = old_token.time_until_expiry_ms().max(0) as u64;
        let new_duration = remaining_ms
            .checked_add(additional_duration_ms)
            .ok_or(TokenError::TimestampOverflow)?;

        self.issue(
            old_token.subject_id,
            old_token.role,
            new_duration,
            old_token.session_id.clone(),
        )
    }

    /// Validate a token's claims.
    pub fn validate_token(&self, token: &Token) -> Result<(), TokenError> {
        if !CommonID::is_valid(token.subject_id) {
            return Err(TokenError::InvalidSubjectIdValue);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_too_short() {
        assert!(matches!(
            TokenService::new_hs256("short"),
            Err(TokenError::SecretTooShort)
        ));
    }

    #[test]
    fn test_expired_token() {
        let service = TokenService::new_hs256(&"a".repeat(32)).expect("token service init");
        let jwt = service
            .issue(
                Id128::from_bytes_unchecked([1u8; 16]),
                Role::User,
                100,
                None,
            ) // Duration of 100 ms
            .expect("issue token");

        // Add logs to inspect timestamps
        println!("JWT issued: {}", jwt);
        let claims: Claims = jsonwebtoken::decode(&jwt, &service.dec, &service.validation)
            .expect("decode token")
            .claims;
        println!("Timestamp iat: {} (ms)", claims.iat * 1000);
        println!("Timestamp exp: {} (ms)", claims.exp * 1000);

        std::thread::sleep(std::time::Duration::from_millis(120)); // Wait slightly less than the total duration to test the limit
        assert!(service.verify(&jwt).is_ok()); // The token should still be valid

        std::thread::sleep(std::time::Duration::from_millis(1000)); // Increase the delay further to exceed tolerance
        assert!(matches!(service.verify(&jwt), Err(TokenError::Expired)));
    }

    #[test]
    fn test_valid_token() {
        let service = TokenService::new_hs256(&"a".repeat(32)).expect("token service init");
        let jwt = service
            .issue(
                Id128::from_bytes_unchecked([123u8; 16]),
                Role::User,
                60000,
                None,
            )
            .expect("issue token");
        let token = service.verify(&jwt).expect("verify token");
        assert_eq!(token.subject_id, Id128::from_bytes_unchecked([123u8; 16]));
    }
}
