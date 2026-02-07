// projects/products/stable/accounts/backend/src/store/tests/audit_buffer.rs
use crate::store::audit_buffer::AuditBuffer;
use crate::store::audit_buffer_config::AuditBufferConfig;
use crate::store::audit_entry::AuditEntry;
use common_time::timestamp_utils::current_timestamp_ms;
use std::path::PathBuf;
use tokio::time::Duration;

use super::helpers::{create_unique_temp_dir, poll_until_async, TestResult};

async fn read_audit_log(path: &PathBuf) -> TestResult<Vec<String>> {
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

#[tokio::test]
async fn test_batch_flush_on_size_threshold() {
    let temp_dir = create_unique_temp_dir("audit_test");
    tokio::fs::create_dir_all(&temp_dir).await.expect("Failed to create test directory");
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
        .expect("Failed to append first audit entry");

    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user2".to_string(),
            action: "logout".to_string(),
            target: "target2".to_string(),
            details: None,
        })
        .await
        .expect("Failed to append second audit entry");

    // Should not have flushed yet
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log before threshold");
    assert_eq!(
        lines.len(),
        0,
        "Should not flush before batch size threshold"
    );

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
        .expect("Failed to append third audit entry");

    // Should have flushed all 3 entries
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log after threshold");
    assert_eq!(lines.len(), 3, "Should flush all 3 entries at threshold");
    assert!(lines[0].contains("user1"), "First entry should contain user1");
    assert!(lines[1].contains("user2"), "Second entry should contain user2");
    assert!(lines[2].contains("user3"), "Third entry should contain user3");

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
}

#[tokio::test]
async fn test_periodic_flush() {
    let temp_dir = create_unique_temp_dir("audit_test");
    tokio::fs::create_dir_all(&temp_dir).await.expect("Failed to create test directory");
    let audit_path = temp_dir.join("audit.log");

    // Configure short flush interval for testing
    let config = AuditBufferConfig {
        max_batch_size: 1000,   // Large batch to test periodic flush only
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
        .expect("Failed to append first audit entry");

    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user2".to_string(),
            action: "logout".to_string(),
            target: "target2".to_string(),
            details: None,
        })
        .await
        .expect("Failed to append second audit entry");

    // Should not have flushed immediately
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log before periodic flush");
    assert_eq!(
        lines.len(),
        0,
        "Should not flush immediately before interval"
    );

    // Poll for periodic flush (2s interval + buffer)
    poll_until_async(
        || async {
            read_audit_log(&audit_path)
                .await
                .map(|lines| lines.len() == 2)
                .unwrap_or(false)
        },
        Duration::from_secs(5),
        Duration::from_millis(100),
    )
    .await
    .expect("Audit log should contain 2 entries after periodic flush");

    // Verify final state
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log after periodic flush");
    assert_eq!(lines.len(), 2, "Should flush after periodic interval");
    assert!(lines[0].contains("user1"), "First entry should contain user1");
    assert!(lines[1].contains("user2"), "Second entry should contain user2");

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
}

#[tokio::test]
async fn test_manual_flush() {
    let temp_dir = create_unique_temp_dir("audit_test");
    tokio::fs::create_dir_all(&temp_dir).await.expect("Failed to create test directory");
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
        .expect("Failed to append audit entry");

    // Should not have flushed
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log before manual flush");
    assert_eq!(lines.len(), 0, "Should not flush before manual flush");

    // Manual flush
    buffer.flush().await.expect("Failed to manually flush audit buffer");

    // Should have flushed
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log after manual flush");
    assert_eq!(lines.len(), 1, "Should have 1 entry after manual flush");
    assert!(lines[0].contains("user1"), "Entry should contain user1");

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
}

#[tokio::test]
async fn test_entries_maintain_order() {
    let temp_dir = create_unique_temp_dir("audit_test");
    tokio::fs::create_dir_all(&temp_dir).await.expect("Failed to create test directory");
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
            .expect("Failed to append audit entry in order test");
    }

    // Should flush on 5th entry
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log for order verification");
    assert_eq!(lines.len(), 5, "Should have 5 entries after batch flush");

    // Verify order is maintained
    for i in 1..=5 {
        assert!(
            lines[i - 1].contains(&format!("user{}", i)),
            "Entry {} should be in position {} but got: {}",
            i,
            i - 1,
            lines[i - 1]
        );
    }

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
}

#[tokio::test]
async fn test_empty_flush_is_safe() {
    let temp_dir = create_unique_temp_dir("audit_test");
    tokio::fs::create_dir_all(&temp_dir).await.expect("Failed to create test directory");
    let audit_path = temp_dir.join("audit.log");

    let config = AuditBufferConfig::default();
    let buffer = AuditBuffer::new(audit_path.clone(), config);

    // Flush empty buffer - should not error
    buffer.flush().await.expect("Flushing empty buffer should not error");

    // File should not be created if nothing to write
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log after empty flush");
    assert_eq!(lines.len(), 0, "Empty flush should not create entries");

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
}

#[tokio::test]
async fn test_multiple_flushes() {
    let temp_dir = create_unique_temp_dir("audit_test");
    tokio::fs::create_dir_all(&temp_dir).await.expect("Failed to create test directory");
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
        .expect("Failed to append first entry of first batch");

    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user2".to_string(),
            action: "action2".to_string(),
            target: "target2".to_string(),
            details: None,
        })
        .await
        .expect("Failed to append second entry of first batch");

    // Should have 2 entries
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log after first batch");
    assert_eq!(lines.len(), 2, "Should have 2 entries after first batch");

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
        .expect("Failed to append first entry of second batch");

    buffer
        .append(AuditEntry {
            timestamp_ms: current_timestamp_ms(),
            actor: "user4".to_string(),
            action: "action4".to_string(),
            target: "target4".to_string(),
            details: None,
        })
        .await
        .expect("Failed to append second entry of second batch");

    // Should have 4 entries total (appended, not replaced)
    let lines = read_audit_log(&audit_path).await.expect("Failed to read audit log after second batch");
    assert_eq!(lines.len(), 4, "Should have 4 entries total after second batch");
    assert!(lines[0].contains("user1"), "First entry should contain user1");
    assert!(lines[1].contains("user2"), "Second entry should contain user2");
    assert!(lines[2].contains("user3"), "Third entry should contain user3");
    assert!(lines[3].contains("user4"), "Fourth entry should contain user4");

    // Cleanup
    tokio::fs::remove_dir_all(&temp_dir).await.ok();
}
