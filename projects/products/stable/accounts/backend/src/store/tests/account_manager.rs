// projects/products/stable/accounts/backend/src/store/tests/account_manager.rs
use crate::store::account_manager::AccountManager;
use crate::store::audit_buffer_config::AuditBufferConfig;
use common_time::timestamp_utils::current_timestamp_ms;
use lazy_static::lazy_static;
use protocol::ProtocolId;
use security::Role;
use std::path::{Path, PathBuf};
use tokio::time::Duration;

lazy_static! {
    static ref PASSWORDS: TestPasswords = TestPasswords::new();
}

// Shared counter for unique test directory names
static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);
use super::helpers::{TestResult, create_unique_temp_dir, poll_until_async};

async fn create_test_manager() -> TestResult<AccountManager> {
    let temp_dir = create_unique_temp_dir("accounts_test");
    Ok(AccountManager::load(temp_dir).await?)
}

async fn create_test_manager_with_config(config: AuditBufferConfig) -> TestResult<AccountManager> {
    let temp_dir = create_unique_temp_dir("accounts_test");
    Ok(AccountManager::load_with_config(temp_dir, config).await?)
}

async fn read_audit_log(data_dir: &Path) -> TestResult<Vec<String>> {
    let audit_path: PathBuf = data_dir.join("audit.log");
    let content: String = match tokio::fs::read_to_string(&audit_path).await {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(Box::new(e)),
    };
    Ok(content.lines().map(|s: &str| s.to_string()).collect())
}

// Centralize all passwords in a single global structure
struct TestPasswords {
    test_password: String,
    password1: String,
    password2: String,
}

impl TestPasswords {
    fn new() -> Self {
        Self {
            test_password: std::env::var("TEST_PASSWORD")
                .unwrap_or_else(|_| "default_test_password".to_string()),
            password1: std::env::var("PASSWORD1")
                .unwrap_or_else(|_| "default_password1".to_string()),
            password2: std::env::var("PASSWORD2")
                .unwrap_or_else(|_| "default_password2".to_string()),
        }
    }
}

#[tokio::test]
async fn test_login_sets_dirty_flag() {
    let manager = create_test_manager()
        .await
        .expect("Failed to create test manager");
    let user_id = ProtocolId::default();

    // Create a test user
    manager
        .create(
            user_id,
            &PASSWORDS.test_password,
            Role::User,
            vec![],
            "test_actor",
        )
        .await
        .expect("Failed to create test user");

    // Clear dirty flag after create
    manager.set_dirty(false);
    assert!(
        !manager.is_dirty(),
        "Dirty flag should be false after clearing"
    );

    // Authenticate (login)
    manager
        .authenticate(&user_id, &PASSWORDS.test_password)
        .await
        .expect("Failed to authenticate test user");

    // Check that dirty flag is set
    assert!(manager.is_dirty(), "Dirty flag should be true after login");

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
}

#[tokio::test]
async fn test_flush_if_dirty_saves_data() {
    let manager = create_test_manager()
        .await
        .expect("Failed to create test manager");
    let user_id = ProtocolId::default();

    // Create a test user
    manager
        .create(
            user_id,
            &PASSWORDS.test_password,
            Role::User,
            vec![],
            "test_actor",
        )
        .await
        .expect("Failed to create test user");

    // Authenticate to update last_login_ms
    manager
        .authenticate(&user_id, &PASSWORDS.test_password)
        .await
        .expect("Failed to authenticate test user");

    // Get last_login_ms before flush
    let user_before = manager
        .get(&user_id)
        .await
        .expect("Failed to get user before flush");
    assert!(
        user_before.last_login_ms.is_some(),
        "last_login_ms should be set after authentication"
    );
    let login_time = user_before.last_login_ms.unwrap();

    // Flush the dirty data
    assert!(manager.is_dirty(), "Manager should be dirty before flush");
    manager
        .flush_if_dirty()
        .await
        .expect("Failed to flush dirty data");
    assert!(
        !manager.is_dirty(),
        "Manager should not be dirty after flush"
    );

    // Reload from disk
    let data_dir = manager.data_dir().clone();
    drop(manager);
    let reloaded = AccountManager::load(data_dir.clone())
        .await
        .expect("Failed to reload manager from disk");

    // Verify last_login_ms persisted
    let user_after = reloaded
        .get(&user_id)
        .await
        .expect("Failed to get user after reload");
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
    let manager = create_test_manager()
        .await
        .expect("Failed to create test manager");

    // Ensure dirty flag is false
    manager.set_dirty(false);

    // Call flush_if_dirty when clean - should not error
    manager
        .flush_if_dirty()
        .await
        .expect("flush_if_dirty should not error when clean");

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
}

#[tokio::test]
async fn test_last_login_survives_restart() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = create_unique_temp_dir();
    tokio::fs::create_dir_all(&temp_dir).await?;

    let user_id = ProtocolId::default();

    // First session: create user and login
    {
        let manager = AccountManager::load(temp_dir.clone()).await?;
        manager
            .create(
                user_id,
                &PASSWORDS.test_password,
                Role::User,
                vec![],
                "test_actor",
            )
            .await?;

        manager
            .authenticate(&user_id, &PASSWORDS.test_password)
            .await?;
        let user = manager.get(&user_id).await?;
        assert!(
            user.last_login_ms.is_some(),
            "last_login_ms should be set after login"
        );

        // Flush to disk (simulate periodic flush)
        manager.flush_if_dirty().await?;
    }

    // Second session: reload and verify persistence
    {
        let manager = AccountManager::load(temp_dir.clone()).await?;
        let user = manager.get(&user_id).await?;
        assert!(
            user.last_login_ms.is_some(),
            "last_login_ms should survive restart after flush"
        );
    }

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
    Ok(())
}

#[tokio::test]
async fn test_audit_entries_batched() {
    let config = AuditBufferConfig {
        max_batch_size: 3,
        flush_interval_secs: 3600, // Long interval to test batch size
    };
    let manager = create_test_manager_with_config(config)
        .await
        .expect("Failed to create test manager with config");
    let user_id1 = ProtocolId::default();
    let user_id2 = ProtocolId::new(common::Id128::new(1, Some(0), Some(0)));

    // Create first user - adds 1 audit entry
    manager
        .create(user_id1, &PASSWORDS.password1, Role::User, vec![], "admin")
        .await
        .expect("Failed to create first test user");

    // Create second user - adds 2nd audit entry
    manager
        .create(user_id2, &PASSWORDS.password2, Role::User, vec![], "admin")
        .await
        .expect("Failed to create second test user");

    // Check audit log - should still be buffered (only 2 entries)
    let lines = read_audit_log(manager.data_dir())
        .await
        .expect("Failed to read audit log before threshold");
    assert_eq!(
        lines.len(),
        0,
        "Should not flush before batch size threshold"
    );

    // Login to trigger 3rd audit entry and flush
    manager
        .authenticate(&user_id1, &PASSWORDS.password1) // Use the same password as during creation
        .await
        .expect("Failed to authenticate test user");

    // Poll until flush completes (with timeout)
    poll_until_async(
        || async {
            read_audit_log(manager.data_dir())
                .await
                .map(|lines| lines.len() == 3)
                .unwrap_or(false)
        },
        Duration::from_secs(2),
        Duration::from_millis(10),
    )
    .await
    .expect("Audit log should flush all 3 entries at threshold");

    // Verify final state
    let lines = read_audit_log(manager.data_dir())
        .await
        .expect("Failed to read audit log after threshold");
    assert_eq!(lines.len(), 3, "Should flush all 3 entries at threshold");

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
}

#[tokio::test]
async fn test_audit_manual_flush() -> Result<(), Box<dyn std::error::Error>> {
    let config = AuditBufferConfig {
        max_batch_size: 1000,
        flush_interval_secs: 3600,
    };
    let manager = create_test_manager_with_config(config).await?;
    let user_id = ProtocolId::default();

    // Create user
    manager
        .create(
            user_id,
            &PASSWORDS.test_password,
            Role::User,
            vec![],
            "admin",
        )
        .await?;

    // Should still be buffered
    let lines = read_audit_log(manager.data_dir()).await?;
    assert_eq!(lines.len(), 0);

    // Manual flush
    manager.flush_audit().await?;

    // Should now be written
    let lines = read_audit_log(manager.data_dir()).await?;
    assert_eq!(lines.len(), 1);
    assert!(lines[0].contains("create"));

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
    Ok(())
}

#[tokio::test]
async fn test_audit_periodic_flush() {
    let config = AuditBufferConfig {
        max_batch_size: 1000,
        flush_interval_secs: 2, // 2 seconds
    };
    let manager = create_test_manager_with_config(config)
        .await
        .expect("Failed to create test manager with config");
    let user_id = ProtocolId::default();

    // Create user
    manager
        .create(user_id, "password", Role::User, vec![], "admin")
        .await
        .expect("Failed to create test user");

    // Should still be buffered
    let lines = read_audit_log(manager.data_dir())
        .await
        .expect("Failed to read audit log before periodic flush");
    assert_eq!(
        lines.len(),
        0,
        "Audit entries should be buffered before periodic flush"
    );

    // Poll for periodic flush (2s interval + buffer time)
    poll_until_async(
        || async {
            read_audit_log(manager.data_dir())
                .await
                .map(|lines| lines.len() == 1)
                .unwrap_or(false)
        },
        Duration::from_secs(5),
        Duration::from_millis(100),
    )
    .await
    .expect("Audit log should contain 1 entry after periodic flush");

    // Verify final state
    let lines = read_audit_log(manager.data_dir())
        .await
        .expect("Failed to read audit log after periodic flush");
    assert_eq!(lines.len(), 1, "Should have 1 entry after periodic flush");
    assert!(
        lines[0].contains("create"),
        "Entry should contain create action"
    );

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
}

#[tokio::test]
async fn test_audit_entries_maintain_order() {
    let config = AuditBufferConfig {
        max_batch_size: 1000,
        flush_interval_secs: 3600,
    };
    let manager = create_test_manager_with_config(config)
        .await
        .expect("Failed to create test manager with config");

    // Create multiple users with different IDs
    for i in 1..=5 {
        let user_id = ProtocolId::new(common::Id128::new(i as u16, Some(0), Some(0)));
        manager
            .create(user_id, "password", Role::User, vec![], "admin")
            .await
            .expect("Failed to create test user in order test");
    }

    // Flush manually
    manager
        .flush_audit()
        .await
        .expect("Failed to manually flush audit in order test");

    // Poll until flush completes
    poll_until_async(
        || async {
            read_audit_log(manager.data_dir())
                .await
                .map(|lines| lines.len() == 5)
                .unwrap_or(false)
        },
        Duration::from_secs(2),
        Duration::from_millis(10),
    )
    .await
    .expect("Audit log should contain 5 entries after manual flush");

    // Verify order - all entries should be present
    let lines = read_audit_log(manager.data_dir())
        .await
        .expect("Failed to read audit log for order verification");
    assert_eq!(lines.len(), 5, "Should have 5 audit entries");

    // All entries should contain "create" action
    for line in &lines {
        assert!(
            line.contains("create"),
            "Each audit entry should be a create action"
        );
    }

    // Cleanup
    tokio::fs::remove_dir_all(manager.data_dir()).await.ok();
}
