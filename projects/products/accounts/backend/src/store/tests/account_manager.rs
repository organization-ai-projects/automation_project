// projects/products/accounts/backend/src/store/tests/account_manager.rs
#[cfg(test)]
mod tests {
    use crate::store::account_manager::AccountManager;
    use common_time::timestamp_utils::current_timestamp_ms;
    use protocol::ProtocolId;
    use security::Role;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

    // Shared counter for unique test directory names
    static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn create_unique_temp_dir() -> PathBuf {
        let id = TEST_DIR_COUNTER.fetch_add(1, AtomicOrdering::Relaxed);
        std::env::temp_dir().join(format!("accounts_test_{}_{}", current_timestamp_ms(), id))
    }

    async fn create_test_manager() -> AccountManager {
        let temp_dir = create_unique_temp_dir();
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();
        AccountManager::load(temp_dir).await.unwrap()
    }

    #[tokio::test]
    async fn test_login_sets_dirty_flag() {
        let manager = create_test_manager().await;
        let user_id = ProtocolId::default();

        // Create a test user
        manager
            .create(user_id, "test_password", Role::User, vec![], "test_actor")
            .await
            .unwrap();

        // Clear dirty flag after create
        manager.set_dirty(false);
        assert!(!manager.is_dirty(), "Dirty flag should be false initially");

        // Authenticate (login)
        manager
            .authenticate(&user_id, "test_password")
            .await
            .unwrap();

        // Check that dirty flag is set
        assert!(manager.is_dirty(), "Dirty flag should be true after login");

        // Cleanup
        tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
    }

    #[tokio::test]
    async fn test_flush_if_dirty_saves_data() {
        let manager = create_test_manager().await;
        let user_id = ProtocolId::default();

        // Create a test user
        manager
            .create(user_id, "test_password", Role::User, vec![], "test_actor")
            .await
            .unwrap();

        // Authenticate to update last_login_ms
        manager
            .authenticate(&user_id, "test_password")
            .await
            .unwrap();

        // Get last_login_ms before flush
        let user_before = manager.get(&user_id).await.unwrap();
        assert!(
            user_before.last_login_ms.is_some(),
            "last_login_ms should be set"
        );
        let login_time = user_before.last_login_ms.unwrap();

        // Flush the dirty data
        assert!(manager.is_dirty(), "Should be dirty before flush");
        manager.flush_if_dirty().await.unwrap();
        assert!(!manager.is_dirty(), "Should not be dirty after flush");

        // Reload from disk
        let data_dir = manager.data_dir().clone();
        drop(manager);
        let reloaded = AccountManager::load(data_dir.clone()).await.unwrap();

        // Verify last_login_ms persisted
        let user_after = reloaded.get(&user_id).await.unwrap();
        assert_eq!(
            user_after.last_login_ms,
            Some(login_time),
            "last_login_ms should persist across reload"
        );

        // Cleanup
        tokio::fs::remove_dir_all(&data_dir).await.ok();
    }

    #[tokio::test]
    async fn test_flush_if_dirty_skips_when_clean() {
        let manager = create_test_manager().await;

        // Ensure dirty flag is false
        manager.set_dirty(false);

        // Call flush_if_dirty when clean - should not error
        manager.flush_if_dirty().await.unwrap();

        // Cleanup
        tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
    }

    #[tokio::test]
    async fn test_last_login_survives_restart() {
        let temp_dir = create_unique_temp_dir();
        tokio::fs::create_dir_all(&temp_dir).await.unwrap();

        let user_id = ProtocolId::default();

        // First session: create user and login
        {
            let manager = AccountManager::load(temp_dir.clone()).await.unwrap();
            manager
                .create(user_id, "test_password", Role::User, vec![], "test_actor")
                .await
                .unwrap();

            manager
                .authenticate(&user_id, "test_password")
                .await
                .unwrap();
            let user = manager.get(&user_id).await.unwrap();
            assert!(
                user.last_login_ms.is_some(),
                "last_login_ms should be set after login"
            );

            // Flush to disk (simulate periodic flush)
            manager.flush_if_dirty().await.unwrap();
        }

        // Second session: reload and verify persistence
        {
            let manager = AccountManager::load(temp_dir.clone()).await.unwrap();
            let user = manager.get(&user_id).await.unwrap();
            assert!(
                user.last_login_ms.is_some(),
                "last_login_ms should survive restart after flush"
            );
        }

        // Cleanup
        tokio::fs::remove_dir_all(&temp_dir).await.ok();
    }
}
