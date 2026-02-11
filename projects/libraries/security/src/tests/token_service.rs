use crate::{Claims, Role, TokenError, TokenService};
use common_time::timestamp_utils::current_timestamp_ms;
use jsonwebtoken::{Algorithm, EncodingKey, Header};

use super::helpers::test_protocol_id;

#[test]
fn test_secret_too_short() {
    assert!(matches!(
        TokenService::new_hs256("short"),
        Err(TokenError::SecretTooShort)
    ));
}

#[test]
fn test_expired_token() {
    let secret = "a".repeat(32);
    // Use a 1-second leeway for clock skew handling
    let service = TokenService::new_hs256_with_leeway(&secret, 1).expect("token service init");

    // Create a signed JWT that is already expired (including leeway).
    let now_s = current_timestamp_ms() / 1_000;
    let claims = Claims {
        sub: test_protocol_id(1),
        jti: test_protocol_id(99),
        role: Role::User,
        iat: now_s.saturating_sub(10),
        exp: now_s.saturating_sub(2),
        sid: None,
    };
    let jwt = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("encode expired token");

    assert!(matches!(service.verify(&jwt), Err(TokenError::Expired)));
}

#[test]
fn test_valid_token() {
    let service = TokenService::new_hs256(&"a".repeat(32)).expect("token service init");
    let jwt = service
        .issue(test_protocol_id(123), Role::User, 60000, None)
        .expect("issue token");
    let token = service.verify(&jwt).expect("verify token");
    assert_eq!(token.subject_id, test_protocol_id(123));
}

#[test]
fn test_issue_sets_expected_iat_exp_relationship() {
    let service = TokenService::new_hs256(&"a".repeat(32)).expect("token service init");
    let jwt = service
        .issue(test_protocol_id(7), Role::User, 100, None)
        .expect("issue token");

    let claims = jsonwebtoken::dangerous::insecure_decode::<Claims>(&jwt)
        .expect("decode claims")
        .claims;

    // `issue()` stores iat in seconds and rounds expiration up with +1 second.
    // For a very short token (100ms), exp should be at least iat+1 and at most iat+2
    // when issuance happens close to a second boundary.
    assert!(claims.exp > claims.iat);
    let exp_delta = claims.exp.saturating_sub(claims.iat);
    assert!(
        (1..=2).contains(&exp_delta),
        "unexpected exp/iat delta for short duration token: {}",
        exp_delta
    );
}
