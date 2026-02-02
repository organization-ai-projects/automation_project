// projects/libraries/identity/src/tests/user_store.rs
use common::Id128;
use protocol::ProtocolId;
use security::Role;

use crate::{IdentityError, UserId, UserStore};

#[tokio::test]
async fn test_add_and_authenticate_user() {
    let store = UserStore::new();
    let user_id = UserId::new(ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16])))
        .expect("create user id for test");

    store
        .add_user(user_id.clone(), "secure_password", Role::User)
        .await
        .expect("add user");

    let role = store
        .authenticate(&user_id, "secure_password")
        .await
        .expect("authenticate user");
    assert_eq!(role, Role::User);
}

#[tokio::test]
async fn test_invalid_password() {
    let store = UserStore::new();
    let user_id = UserId::new(ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16])))
        .expect("create user id for test");

    store
        .add_user(user_id.clone(), "correct_password", Role::User)
        .await
        .expect("add user");

    let result = store.authenticate(&user_id, "wrong_password").await;
    assert!(matches!(result, Err(IdentityError::InvalidCredentials)));
}

#[tokio::test]
async fn test_user_not_found() {
    let store = UserStore::new();
    let user_id = UserId::new(ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16])))
        .expect("create user id for test");

    let result = store.authenticate(&user_id, "any_password").await;
    assert!(matches!(result, Err(IdentityError::InvalidCredentials)));
}

#[tokio::test]
async fn test_empty_password() {
    let store = UserStore::new();
    let user_id = UserId::new(ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16])))
        .expect("create user id for test");

    let result = store.add_user(user_id, "", Role::User).await;
    assert!(matches!(result, Err(IdentityError::EmptyPassword)));
}

#[tokio::test]
async fn test_user_exists() {
    let store = UserStore::new();
    let user_id = UserId::new(ProtocolId::new(Id128::from_bytes_unchecked([1u8; 16])))
        .expect("create user id for test");

    assert!(!store.user_exists(&user_id).await);

    store
        .add_user(user_id.clone(), "password", Role::Admin)
        .await
        .expect("add user");

    assert!(store.user_exists(&user_id).await);
}
