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

    // Poll until token expires (with timeout to prevent hanging)
    // Expiry calculation: 100ms token duration rounds up to 1s (see token_service.rs line 72)
    // Plus 1s leeway = 2s total grace period from token issue time
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(5); // Generous timeout
    let mut expired = false;

    while start.elapsed() < timeout {
        if matches!(service.verify(&jwt), Err(TokenError::Expired)) {
            expired = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    assert!(
        expired,
        "Token should have expired within timeout, but verification still succeeds after {:?}",
        start.elapsed()
    );
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
