// projects/products/stable/accounts/backend/src/store/tests/audit_buffer.rs
use crate::store::audit_buffer::AuditBuffer;
use crate::store::audit_buffer_config::AuditBufferConfig;
use crate::store::audit_entry::AuditEntry;
use common_time::timestamp_utils::current_timestamp_ms;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use tokio::time::{sleep, Duration};

// Shared counter for unique test directory names
static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn create_unique_temp_dir() -> PathBuf {
    let id = TEST_DIR_COUNTER.fetch_add(1, AtomicOrdering::Relaxed);
    std::env::temp_dir().join(format!("audit_test_{}_{}", current_timestamp_ms(), id))
}

async fn read_audit_log(path: &PathBuf) -> Vec<String> {
    if !path.exists() {
    return vec![];
    }
    let content = tokio::fs::read_to_string(path).await.unwrap();
    content.lines().map(|s| s.to_string()).collect()
}

#[tokio::test]
async fn test_batch_flush_on_size_threshold() {
    let temp_dir = create_unique_temp_dir();
    tokio::fs::create_dir_all(&temp_dir).await.unwrap();
    let audit_path = temp_dir.join("audit.log");

    // Configure small batch size for testing
    let config = AuditBufferConfig {
        max_batch_size: 3,
        flush_interval_secs: 3600, // Long interval to test batch size only
    };

    let buffer = AuditBuffer::new(audit_path.clone(), config);

    // Add 2 entries - should not flush yet
    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user1".to_string(),
            action: "login".to_string(),
            target: "target1".to_string(),
            details: None,
        })
        .await
        .unwrap();

    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user2".to_string(),
            action: "logout".to_string(),
            target: "target2".to_string(),
            details: None,
        })
        .await
        .unwrap();

    // Should not have flushed yet
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(lines.len(), 0, "Should not flush before batch size threshold");

    // Add 3rd entry - should trigger flush
    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user3".to_string(),
            action: "create".to_string(),
            target: "target3".to_string(),
            details: None,
        })
        .await
        .unwrap();

    // Should have flushed all 3 entries
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(lines.len(), 3, "Should flush all 3 entries at threshold");
    assert!(lines[0].contains("user1"));
    assert!(lines[1].contains("user2"));
    assert!(lines[2].contains("user3"));

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
    }

#[tokio::test]
async fn test_periodic_flush() {
    let temp_dir = create_unique_temp_dir();
    tokio::fs::create_dir_all(&temp_dir).await.unwrap();
    let audit_path = temp_dir.join("audit.log");

    // Configure short flush interval for testing
    let config = AuditBufferConfig {
        max_batch_size: 1000, // Large batch to test periodic flush only
        flush_interval_secs: 2, // 2 seconds
    };

    let buffer = AuditBuffer::new(audit_path.clone(), config);

    // Add entries
    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user1".to_string(),
            action: "login".to_string(),
            target: "target1".to_string(),
            details: None,
        })
        .await
        .unwrap();

    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user2".to_string(),
            action: "logout".to_string(),
            target: "target2".to_string(),
            details: None,
        })
        .await
        .unwrap();

    // Should not have flushed immediately
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(
        lines.len(),
        0,
        "Should not flush immediately before interval"
    );

    // Wait for periodic flush (2 seconds + buffer)
    sleep(Duration::from_secs(3)).await;

    // Should have flushed via periodic mechanism
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(lines.len(), 2, "Should flush after periodic interval");
    assert!(lines[0].contains("user1"));
    assert!(lines[1].contains("user2"));

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
    }

#[tokio::test]
async fn test_manual_flush() {
    let temp_dir = create_unique_temp_dir();
    tokio::fs::create_dir_all(&temp_dir).await.unwrap();
    let audit_path = temp_dir.join("audit.log");

    let config = AuditBufferConfig {
        max_batch_size: 1000,
        flush_interval_secs: 3600, // Long interval
    };

    let buffer = AuditBuffer::new(audit_path.clone(), config);

    // Add entry
    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user1".to_string(),
            action: "login".to_string(),
            target: "target1".to_string(),
            details: None,
        })
        .await
        .unwrap();

    // Should not have flushed
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(lines.len(), 0);

    // Manual flush
    buffer.flush().await.unwrap();

    // Should have flushed
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(lines.len(), 1);
    assert!(lines[0].contains("user1"));

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
    }

#[tokio::test]
async fn test_entries_maintain_order() {
    let temp_dir = create_unique_temp_dir();
    tokio::fs::create_dir_all(&temp_dir).await.unwrap();
    let audit_path = temp_dir.join("audit.log");

    let config = AuditBufferConfig {
        max_batch_size: 5,
        flush_interval_secs: 3600,
    };

    let buffer = AuditBuffer::new(audit_path.clone(), config);

    // Add entries in specific order
    for i in 1..=5 {
        buffer
            .append(AuditEntry {
                timestamp_ms: current_timestamp_ms() + i,
                actor: format!("user{}", i),
                action: "action".to_string(),
                target: format!("target{}", i),
                details: None,
            })
            .await
            .unwrap();
    }

    // Should flush on 5th entry
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(lines.len(), 5);

    // Verify order is maintained
    for i in 1..=5 {
        assert!(
            lines[i - 1].contains(&format!("user{}", i)),
            "Entry {} should be in position {}",
            i,
            i - 1
        );
    }

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
    }

#[tokio::test]
async fn test_empty_flush_is_safe() {
    let temp_dir = create_unique_temp_dir();
    tokio::fs::create_dir_all(&temp_dir).await.unwrap();
    let audit_path = temp_dir.join("audit.log");

    let config = AuditBufferConfig::default();
    let buffer = AuditBuffer::new(audit_path.clone(), config);

    // Flush empty buffer - should not error
    buffer.flush().await.unwrap();

    // File should not be created if nothing to write
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(lines.len(), 0);

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
    }

#[tokio::test]
async fn test_multiple_flushes() {
    let temp_dir = create_unique_temp_dir();
    tokio::fs::create_dir_all(&temp_dir).await.unwrap();
    let audit_path = temp_dir.join("audit.log");

    let config = AuditBufferConfig {
        max_batch_size: 2,
        flush_interval_secs: 3600,
    };

    let buffer = AuditBuffer::new(audit_path.clone(), config);

    // First batch
    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user1".to_string(),
            action: "action1".to_string(),
            target: "target1".to_string(),
            details: None,
        })
        .await
        .unwrap();

    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user2".to_string(),
            action: "action2".to_string(),
            target: "target2".to_string(),
            details: None,
        })
        .await
        .unwrap();

    // Should have 2 entries
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(lines.len(), 2);

    // Second batch
    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user3".to_string(),
            action: "action3".to_string(),
            target: "target3".to_string(),
            details: None,
        })
        .await
        .unwrap();

    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user4".to_string(),
            action: "action4".to_string(),
            target: "target4".to_string(),
            details: None,
        })
        .await
        .unwrap();

    // Should have 4 entries total (appended, not replaced)
    let lines = read_audit_log(&audit_path).await;
    assert_eq!(lines.len(), 4);
    assert!(lines[0].contains("user1"));
    assert!(lines[1].contains("user2"));
    assert!(lines[2].contains("user3"));
    assert!(lines[3].contains("user4"));

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
    }

