// projects/libraries/layers/domain/identity/src/tests/user_store.rs
// Unit tests for UserStore - internal testing only
// Integration tests in tests/user_store.rs cover the main scenarios

use super::helpers::create_test_user_id;
use crate::UserStore;

#[tokio::test]
async fn test_user_store_new_is_empty() {
    // Test that a new UserStore is empty
    let store = UserStore::new();
    assert_eq!(store.user_count().await, 0);
}

#[tokio::test]
async fn test_user_store_concurrent_access() {
    // Test thread-safe concurrent access to UserStore
    use security_core::Role;

    let store = UserStore::new();
    let user_id = create_test_user_id(1);

    store
        .add_user(user_id.clone(), "password", Role::User)
        .await
        .expect("failed to add user");

    // Simulate concurrent reads and verify correctness
    let store_clone1 = store.clone();
    let store_clone2 = store.clone();
    let user_id_clone1 = user_id.clone();
    let user_id_clone2 = user_id.clone();

    let handle1 = tokio::spawn(async move {
        let exists = store_clone1.user_exists(&user_id_clone1).await;
        assert!(exists, "user should exist in task 1");
    });

    let handle2 = tokio::spawn(async move {
        let exists = store_clone2.user_exists(&user_id_clone2).await;
        assert!(exists, "user should exist in task 2");
    });

    handle1.await.expect("task 1 panicked");
    handle2.await.expect("task 2 panicked");
}
