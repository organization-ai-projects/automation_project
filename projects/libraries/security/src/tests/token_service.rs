use crate::{Role, TokenError, TokenService};

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
    // Use a 1-second leeway for clock skew handling
    let service =
        TokenService::new_hs256_with_leeway(&"a".repeat(32), 1).expect("token service init");
    
    // Issue a very short-lived token (100ms actual duration)
    let jwt = service
        .issue(test_protocol_id(1), Role::User, 100, None)
        .expect("issue token");

    // Immediately verify - should succeed
    assert!(service.verify(&jwt).is_ok());

    // Sleep for 500ms - still within grace period
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(service.verify(&jwt).is_ok());

    // Sleep for 3 more seconds - well past expiry + leeway with very comfortable margin
    // Token rounds to 1s expiry, leeway is 1s, so should expire around 2s
    // We wait 3.5s total to have plenty of margin for slow CI and timing variations
    std::thread::sleep(std::time::Duration::from_millis(3000));

    // Should now be expired
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
