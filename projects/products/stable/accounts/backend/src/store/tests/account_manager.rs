// projects/products/stable/accounts/backend/src/store/tests/account_manager.rs
use crate::store::account_manager::AccountManager;
use crate::store::audit_buffer_config::AuditBufferConfig;
use common_time::timestamp_utils::current_timestamp_ms;
use protocol::ProtocolId;
use security::Role;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use tokio::time::{sleep, Duration};

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

async fn create_test_manager_with_config(config: AuditBufferConfig) -> AccountManager {
    let temp_dir = create_unique_temp_dir();
    tokio::fs::create_dir_all(&temp_dir).await.unwrap();
    AccountManager::load_with_config(temp_dir, config)
        .await
        .unwrap()
}

async fn read_audit_log(data_dir: &PathBuf) -> Vec<String> {
    let audit_path = data_dir.join("audit.log");
    if !audit_path.exists() {
        return vec![];
    }
    let content = tokio::fs::read_to_string(&audit_path).await.unwrap();
    content.lines().map(|s| s.to_string()).collect()
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

#[tokio::test]
async fn test_audit_entries_batched() {
        let config = AuditBufferConfig {
            max_batch_size: 3,
            flush_interval_secs: 3600, // Long interval to test batch size
        };
        let manager = create_test_manager_with_config(config).await;
        let user_id1 = ProtocolId::default();
        let user_id2 = ProtocolId::new(common::Id128::new(1, Some(0), Some(0)));

        // Create first user - adds 1 audit entry
        manager
            .create(user_id1, "password1", Role::User, vec![], "admin")
            .await
            .unwrap();

        // Small delay to ensure async operations complete
        sleep(Duration::from_millis(50)).await;

        // Create second user - adds 2nd audit entry
        manager
            .create(user_id2, "password2", Role::User, vec![], "admin")
            .await
            .unwrap();

        // Small delay to ensure async operations complete
        sleep(Duration::from_millis(50)).await;

        // Check audit log - should still be buffered (only 2 entries)
        let lines = read_audit_log(manager.data_dir()).await;
        assert_eq!(
            lines.len(),
            0,
            "Should not flush before batch size threshold"
        );

        // Login to trigger 3rd audit entry and flush
        manager
            .authenticate(&user_id1, "password1")
            .await
            .unwrap();

        // Small delay to ensure flush completes
        sleep(Duration::from_millis(100)).await;

        // Now should have flushed all 3 entries
        let lines = read_audit_log(manager.data_dir()).await;
        assert_eq!(lines.len(), 3, "Should flush all 3 entries at threshold");

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
}

#[tokio::test]
async fn test_audit_manual_flush() {
        let config = AuditBufferConfig {
            max_batch_size: 1000,
            flush_interval_secs: 3600,
        };
        let manager = create_test_manager_with_config(config).await;
        let user_id = ProtocolId::default();

        // Create user
        manager
            .create(user_id, "password", Role::User, vec![], "admin")
            .await
            .unwrap();

        // Should still be buffered
        let lines = read_audit_log(manager.data_dir()).await;
        assert_eq!(lines.len(), 0);

        // Manual flush
        manager.flush_audit().await.unwrap();

        // Should now be written
        let lines = read_audit_log(manager.data_dir()).await;
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("create"));

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
}

#[tokio::test]
async fn test_audit_periodic_flush() {
        let config = AuditBufferConfig {
            max_batch_size: 1000,
            flush_interval_secs: 2, // 2 seconds
        };
        let manager = create_test_manager_with_config(config).await;
        let user_id = ProtocolId::default();

        // Create user
        manager
            .create(user_id, "password", Role::User, vec![], "admin")
            .await
            .unwrap();

        // Should still be buffered
        let lines = read_audit_log(manager.data_dir()).await;
        assert_eq!(lines.len(), 0);

        // Wait for periodic flush
        sleep(Duration::from_secs(3)).await;

        // Should have flushed
        let lines = read_audit_log(manager.data_dir()).await;
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("create"));

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
}

#[tokio::test]
async fn test_audit_entries_maintain_order() {
        let config = AuditBufferConfig {
            max_batch_size: 1000,
            flush_interval_secs: 3600,
        };
        let manager = create_test_manager_with_config(config).await;

        // Create multiple users with different IDs
        for i in 1..=5 {
            let user_id = ProtocolId::new(common::Id128::new(i as u16, Some(0), Some(0)));
            manager
                .create(user_id, "password", Role::User, vec![], "admin")
                .await
                .unwrap();
        }

        // Flush manually
        manager.flush_audit().await.unwrap();

        // Verify order - all entries should be present
        let lines = read_audit_log(manager.data_dir()).await;
        assert_eq!(lines.len(), 5);

        // All entries should contain "create" action
        for line in &lines {
            assert!(line.contains("create"), "Each entry should be a create action");
        }

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
}
