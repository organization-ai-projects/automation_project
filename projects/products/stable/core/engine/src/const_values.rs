// projects/products/core/engine/src/const_values.rs
use std::time::Duration;

pub(crate) const WS_MAX_MESSAGE_BYTES: usize = 256 * 1024; // 256 KiB
pub(crate) const WS_PING_EVERY: Duration = Duration::from_secs(20);
pub(crate) const WS_IDLE_TIMEOUT: Duration = Duration::from_secs(60);
pub(crate) const LOGIN_MAX_DURATION_MS: u64 = 7 * 24 * 60 * 60 * 1000; // 7 days max
pub(crate) const DEFAULT_DURATION_MS: u64 = 60 * 60 * 1000; // 1 hour
