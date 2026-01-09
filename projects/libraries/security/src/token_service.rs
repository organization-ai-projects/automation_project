// projects/libraries/security/src/token_service.rs
use crate::auth::UserId;
use crate::{Claims, Role, Token, TokenError};
use common::common_id::is_valid_id;
use common_time::timestamp_utils::current_timestamp_ms;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

/// Service pour issuer/verify des JWT.
/// - Stateless: pas besoin de store.
#[derive(Clone)]
pub struct TokenService {
    enc: EncodingKey,
    dec: DecodingKey,
    validation: Validation,
}

impl TokenService {
    /// secret: string robuste (ENV). Min 32 chars recommandé.
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

        Ok(Self {
            enc: EncodingKey::from_secret(s.as_bytes()),
            dec: DecodingKey::from_secret(s.as_bytes()),
            validation,
        })
    }

    /// Issue a signed JWT. duration_ms must be > 0.
    pub fn issue(
        &self,
        user_id: UserId,
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
        let exp_s = exp_ms / 1000;

        let claims = Claims {
            sub: user_id.to_string(),
            jti: Uuid::now_v7().to_string(),
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

        // Hardening: validate sub numeric + is_valid_id even after decode
        let user_id = UserId::from(c.sub.as_str());
        if !is_valid_id(user_id.value()) {
            return Err(TokenError::InvalidUserIdValue);
        }

        let issued_at_ms = c.iat.saturating_mul(1000);
        let expires_at_ms = c.exp.saturating_mul(1000);

        Ok(Token {
            value: c.jti,
            user_id,
            role: c.role,
            issued_at_ms,
            expires_at_ms,
            session_id: c.sid,
        })
    }

    /// Renouvelle un token existant (crée un nouveau JWT signé)
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
            old_token.user_id.clone(),
            old_token.role,
            new_duration,
            old_token.session_id.clone(),
        )
    }

    /// Validate a token's claims.
    pub fn validate_token(&self, token: &Token) -> Result<(), TokenError> {
        if !is_valid_id(token.user_id.value()) {
            return Err(TokenError::InvalidUserIdValue);
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
        let service = TokenService::new_hs256(&"a".repeat(32)).unwrap();
        let jwt = service
            .issue(UserId::from("123"), Role::User, 1, None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(matches!(service.verify(&jwt), Err(TokenError::Expired)));
    }

    #[test]
    fn test_valid_token() {
        let service = TokenService::new_hs256(&"a".repeat(32)).unwrap();
        let jwt = service
            .issue(UserId::from("123"), Role::User, 60000, None)
            .unwrap();
        let token = service.verify(&jwt).unwrap();
        assert_eq!(token.user_id.value(), 123);
    }
}