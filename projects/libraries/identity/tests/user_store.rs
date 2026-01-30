use common::Id128;
use identity::{IdentityError, UserId, UserStore};
use security::Role;

#[tokio::test]
async fn add_and_authenticate_user() {
    let store = UserStore::new();
    let user_id = match UserId::new(Id128::from_bytes_unchecked([1u8; 16])) {
        Ok(id) => id,
        Err(e) => panic!("Failed to create UserId: {}", e),
    };

    store
        .add_user(user_id.clone(), "secure_password", Role::User)
        .await
        .expect("Failed to add user");

    let role = store
        .authenticate(&user_id, "secure_password")
        .await
        .expect("Failed to authenticate user");
    assert_eq!(role, Role::User);
}

#[tokio::test]
async fn invalid_password_is_rejected() {
    let store = UserStore::new();
    let user_id = match UserId::new(Id128::from_bytes_unchecked([2u8; 16])) {
        Ok(id) => id,
        Err(e) => panic!("Failed to create UserId: {}", e),
    };

    store
        .add_user(user_id.clone(), "correct_password", Role::User)
        .await
        .expect("Failed to add user");

    let result = store.authenticate(&user_id, "wrong_password").await;
    assert!(matches!(result, Err(IdentityError::InvalidCredentials)));
}

#[tokio::test]
async fn missing_user_is_rejected() {
    let store = UserStore::new();
    let user_id = match UserId::new(Id128::from_bytes_unchecked([3u8; 16])) {
        Ok(id) => id,
        Err(e) => panic!("Failed to create UserId: {}", e),
    };

    let result = store.authenticate(&user_id, "any_password").await;
    assert!(matches!(result, Err(IdentityError::InvalidCredentials)));
}

#[tokio::test]
async fn empty_password_is_rejected() {
    let store = UserStore::new();
    let user_id = match UserId::new(Id128::from_bytes_unchecked([4u8; 16])) {
        Ok(id) => id,
        Err(e) => panic!("Failed to create UserId: {}", e),
    };

    let result = store.add_user(user_id, "", Role::User).await;
    assert!(matches!(result, Err(IdentityError::EmptyPassword)));
}

#[tokio::test]
async fn user_exists_and_count_work() {
    let store = UserStore::new();
    let user_id = match UserId::new(Id128::from_bytes_unchecked([5u8; 16])) {
        Ok(id) => id,
        Err(e) => panic!("Failed to create UserId: {}", e),
    };

    assert_eq!(store.user_count().await, 0);
    assert!(!store.user_exists(&user_id).await);

    store
        .add_user(user_id.clone(), "password", Role::Admin)
        .await
        .expect("Failed to add user");

    assert!(store.user_exists(&user_id).await);
    assert_eq!(store.user_count().await, 1);
}

#[tokio::test]
async fn get_user_role_returns_role() {
    let store = UserStore::new();
    let user_id = match UserId::new(Id128::from_bytes_unchecked([6u8; 16])) {
        Ok(id) => id,
        Err(e) => panic!("Failed to create UserId: {}", e),
    };

    assert!(store.get_user_role(&user_id).await.is_none());

    store
        .add_user(user_id.clone(), "password", Role::Moderator)
        .await
        .expect("Failed to add user");

    assert_eq!(store.get_user_role(&user_id).await, Some(Role::Moderator));
}
