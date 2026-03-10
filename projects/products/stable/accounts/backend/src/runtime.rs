//! projects/products/stable/accounts/backend/src/runtime.rs
use tokio::task;
use tracing::warn;

use crate::store::AccountManager;

pub async fn flush_and_stop_periodic_task(
    flush_handle: &task::JoinHandle<()>,
    manager: &AccountManager,
    context: &str,
) {
    flush_handle.abort();
    if manager.is_dirty()
        && let Err(err) = manager.flush_if_dirty().await
    {
        warn!(%err, "Failed to flush accounts {}", context);
    }
    if let Err(err) = manager.flush_audit().await {
        warn!(%err, "Failed to flush audit log {}", context);
    }
}
