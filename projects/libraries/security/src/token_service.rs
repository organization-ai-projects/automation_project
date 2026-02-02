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
        Self::new_hs256_with_leeway(secret, 0)
    }

    /// secret: robust string (ENV). Minimum 32 characters recommended.
    /// leeway_seconds: grace window allowed after exp for clock skew handling.
    pub fn new_hs256_with_leeway(secret: &str, leeway_seconds: u64) -> Result<Self, TokenError> {
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
        validation.leeway = leeway_seconds;

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

        let data =
            jsonwebtoken::decode::<Claims>(jwt, &self.dec, &self.validation).map_err(|e| {
                if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                    TokenError::Expired
                } else {
                    TokenError::Jwt(e.to_string())
                }
            })?;

        let c = data.claims;

        // Hardening: validate sub numeric + CommonID validation
        let subject_id = Id128::from_hex(&c.sub).map_err(|_| TokenError::InvalidSubjectIdFormat)?;

        if !CommonID::is_valid(subject_id) {
            return Err(TokenError::InvalidSubjectIdValue);
        }

        let issued_at_ms = c.iat.saturating_mul(1000);
        let expires_at_ms = c.exp.saturating_mul(1000);

        // jsonwebtoken Validation already checked `exp` with configured leeway.

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
