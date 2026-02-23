// projects/products/unstable/platform_versioning/backend/src/auth/token_verifier.rs
use crate::auth::{AuthToken, TokenClaims};
use crate::errors::PvError;

/// Verifies bearer tokens and extracts claims.
///
/// # Initial implementation
/// Tokens are HMAC-signed JSON payloads (no external JWT library required).
/// The signing key is provided at construction time and must be at least 32 bytes.
///
/// Future implementations may support asymmetric keys by adding a new variant.
pub struct TokenVerifier {
    secret: Vec<u8>,
}

impl TokenVerifier {
    /// Creates a new verifier with the given secret key.
    pub fn new(secret: impl Into<Vec<u8>>) -> Result<Self, PvError> {
        let secret = secret.into();
        if secret.len() < 32 {
            return Err(PvError::Internal(
                "signing key must be at least 32 bytes".to_string(),
            ));
        }
        Ok(Self { secret })
    }

    /// Issues a signed token encoding `claims`.
    pub fn issue(&self, claims: &TokenClaims) -> Result<AuthToken, PvError> {
        let payload = serde_json::to_string(claims)
            .map_err(|e| PvError::Internal(format!("serialize claims: {e}")))?;
        let sig = self.sign(payload.as_bytes());
        let encoded = format!(
            "{}.{}",
            base64_encode(payload.as_bytes()),
            base64_encode(&sig)
        );
        Ok(AuthToken::new(encoded))
    }

    /// Verifies `token` and returns the decoded [`TokenClaims`].
    pub fn verify(&self, token: &AuthToken) -> Result<TokenClaims, PvError> {
        let raw = token.as_str();
        let dot = raw
            .rfind('.')
            .ok_or_else(|| PvError::AuthRequired("malformed token".to_string()))?;
        let payload_b64 = &raw[..dot];
        let sig_b64 = &raw[dot + 1..];

        let payload_bytes = base64_decode(payload_b64)
            .map_err(|_| PvError::AuthRequired("invalid token encoding".to_string()))?;
        let sig_bytes = base64_decode(sig_b64)
            .map_err(|_| PvError::AuthRequired("invalid token encoding".to_string()))?;

        let expected = self.sign(&payload_bytes);
        if !constant_time_eq(&sig_bytes, &expected) {
            return Err(PvError::AuthRequired("invalid token signature".to_string()));
        }

        let claims: TokenClaims = serde_json::from_slice(&payload_bytes)
            .map_err(|e| PvError::AuthRequired(format!("parse claims: {e}")))?;
        Ok(claims)
    }

    fn sign(&self, data: &[u8]) -> Vec<u8> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(&self.secret)
            .expect("HMAC key length is validated in TokenVerifier::new");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }
}

fn base64_encode(data: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 {
            chunk[1] as usize
        } else {
            0
        };
        let b2 = if chunk.len() > 2 {
            chunk[2] as usize
        } else {
            0
        };
        let _ = write!(out, "{}", TABLE[b0 >> 2] as char);
        let _ = write!(out, "{}", TABLE[((b0 & 3) << 4) | (b1 >> 4)] as char);
        if chunk.len() > 1 {
            let _ = write!(out, "{}", TABLE[((b1 & 0xf) << 2) | (b2 >> 6)] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            let _ = write!(out, "{}", TABLE[b2 & 0x3f] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn base64_decode(s: &str) -> Result<Vec<u8>, ()> {
    const INV: [i8; 256] = {
        let mut t = [-1i8; 256];
        let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut i = 0usize;
        while i < table.len() {
            t[table[i] as usize] = i as i8;
            i += 1;
        }
        t
    };

    let s = s.trim_end_matches('=');
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        let a = INV[bytes[i] as usize];
        let b = INV[bytes[i + 1] as usize];
        if a < 0 || b < 0 {
            return Err(());
        }
        out.push(((a as u8) << 2) | ((b as u8) >> 4));
        if i + 2 < bytes.len() {
            let c = INV[bytes[i + 2] as usize];
            if c < 0 {
                return Err(());
            }
            out.push(((b as u8) << 4) | ((c as u8) >> 2));
        }
        if i + 3 < bytes.len() {
            let d = INV[bytes[i + 3] as usize];
            if d < 0 {
                return Err(());
            }
            out.push(((INV[bytes[i + 2] as usize] as u8) << 6) | (d as u8));
        }
        i += 4;
    }
    Ok(out)
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{Permission, TokenClaims};

    fn make_claims() -> TokenClaims {
        TokenClaims {
            subject: "alice".to_string(),
            grants: vec![],
            expires_at: None,
        }
    }

    fn verifier() -> TokenVerifier {
        TokenVerifier::new(b"a_very_secure_secret_key_here_!!" as &[u8]).unwrap()
    }

    #[test]
    fn issue_and_verify_roundtrip() {
        let v = verifier();
        let claims = make_claims();
        let token = v.issue(&claims).unwrap();
        let decoded = v.verify(&token).unwrap();
        assert_eq!(decoded.subject, "alice");
    }

    #[test]
    fn tampered_token_is_rejected() {
        let v = verifier();
        let claims = make_claims();
        let mut token_str = v.issue(&claims).unwrap().as_str().to_string();
        // Flip a character in the payload.
        let char_idx = token_str.len() / 2;
        let c = token_str.chars().nth(char_idx).unwrap_or('A');
        token_str.replace_range(char_idx..char_idx + 1, if c == 'A' { "B" } else { "A" });
        let result = v.verify(&AuthToken::new(token_str));
        assert!(result.is_err());
    }

    #[test]
    fn short_key_rejected() {
        let result = TokenVerifier::new(b"short");
        assert!(result.is_err());
    }

    #[test]
    fn has_permission_grants_match() {
        use crate::auth::PermissionGrant;
        let repo_id: crate::ids::RepoId = "my-repo".parse().unwrap();
        let claims = TokenClaims {
            subject: "bob".to_string(),
            grants: vec![PermissionGrant {
                repo_id: Some(repo_id.clone()),
                permission: Permission::Read,
            }],
            expires_at: None,
        };
        assert!(claims.has_permission(&repo_id, Permission::Read));
        assert!(!claims.has_permission(&repo_id, Permission::Write));
    }
}
