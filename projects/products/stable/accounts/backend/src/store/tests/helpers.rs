// projects/products/stable/accounts/backend/src/store/tests/helpers.rs
//! Shared test helpers and utilities for accounts-backend tests.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::time::Duration;
use common_time::timestamp_utils::current_timestamp_ms;

/// Type alias for test results to improve consistency and readability.
pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

/// Shared counter for unique test directory names.
static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Creates a unique temporary directory for test isolation.
///
/// Uses a combination of timestamp and atomic counter to ensure uniqueness
/// even when tests run in parallel.
pub fn create_unique_temp_dir(prefix: &str) -> PathBuf {
    let id = TEST_DIR_COUNTER.fetch_add(1, AtomicOrdering::Relaxed);
    std::env::temp_dir().join(format!("{}_{}_{}",
        prefix,
        current_timestamp_ms(),
        id
    ))
}

/// Polls a condition with exponential backoff until it returns true or times out.
///
/// # Arguments
/// * `condition` - A closure that returns true when the desired state is reached
/// * `timeout` - Maximum time to wait before giving up
/// * `initial_delay` - Initial delay between checks
///
/// # Returns
/// `Ok(())` if condition became true, `Err` if timeout was reached
pub async fn poll_until<F>(
    mut condition: F,
    timeout: Duration,
    initial_delay: Duration,
) -> TestResult<()>
where
    F: FnMut() -> bool,
{
    let start = tokio::time::Instant::now();
    let mut delay = initial_delay;
    
    while start.elapsed() < timeout {
        if condition() {
            return Ok(());
        }
        tokio::time::sleep(delay).await;
        // Exponential backoff with max of 500ms
        delay = std::cmp::min(delay * 2, Duration::from_millis(500));
    }
    
    Err("Timeout waiting for condition".into())
}

/// Polls an async condition with exponential backoff until it returns true or times out.
///
/// # Arguments
/// * `condition` - An async closure that returns true when the desired state is reached
/// * `timeout` - Maximum time to wait before giving up
/// * `initial_delay` - Initial delay between checks
///
/// # Returns
/// `Ok(())` if condition became true, `Err` if timeout was reached
pub async fn poll_until_async<F, Fut>(
    mut condition: F,
    timeout: Duration,
    initial_delay: Duration,
) -> TestResult<()>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let start = tokio::time::Instant::now();
    let mut delay = initial_delay;
    
    while start.elapsed() < timeout {
        if condition().await {
            return Ok(());
        }
        tokio::time::sleep(delay).await;
        // Exponential backoff with max of 500ms
        delay = std::cmp::min(delay * 2, Duration::from_millis(500));
    }
    
    Err("Timeout waiting for async condition".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_poll_until_success() {
        let mut counter = 0;
        let result = poll_until(
            || {
                counter += 1;
                counter >= 3
            },
            Duration::from_secs(1),
            Duration::from_millis(10),
        ).await;
        assert!(result.is_ok(), "poll_until should succeed when condition becomes true");
    }

    #[tokio::test]
    async fn test_poll_until_timeout() {
        let result = poll_until(
            || false,
            Duration::from_millis(100),
            Duration::from_millis(10),
        ).await;
        assert!(result.is_err(), "poll_until should timeout when condition never becomes true");
    }

    #[tokio::test]
    async fn test_poll_until_async_success() {
        let counter = Arc::new(AtomicBool::new(false));
        let counter_clone = counter.clone();
        
        // Set flag to true after first check
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            counter_clone.store(true, Ordering::SeqCst);
        });
        
        let result = poll_until_async(
            || async { counter.load(Ordering::SeqCst) },
            Duration::from_secs(1),
            Duration::from_millis(10),
        ).await;
        assert!(result.is_ok(), "poll_until_async should succeed when condition becomes true");
    }

    #[test]
    fn test_create_unique_temp_dir() {
        let dir1 = create_unique_temp_dir("test");
        let dir2 = create_unique_temp_dir("test");
        assert_ne!(dir1, dir2, "Consecutive calls should produce different directories");
    }
}
