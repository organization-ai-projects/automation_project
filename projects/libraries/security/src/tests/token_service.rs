use common::custom_uuid::Id128;

use crate::{Role, TokenError, TokenService};

#[test]
fn test_secret_too_short() {
    assert!(matches!(
        TokenService::new_hs256("short"),
        Err(TokenError::SecretTooShort)
    ));
}

#[test]
fn test_expired_token() {
    let service =
        TokenService::new_hs256_with_leeway(&"a".repeat(32), 1).expect("token service init");
    let jwt = service
        .issue(
            Id128::from_bytes_unchecked([1u8; 16]),
            Role::User,
            100,
            None,
        )
        .expect("issue token");

    std::thread::sleep(std::time::Duration::from_millis(120));
    assert!(service.verify(&jwt).is_ok());

    for _ in 0..30 {
        if matches!(service.verify(&jwt), Err(TokenError::Expired)) {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    panic!("token should be expired after waiting");
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
