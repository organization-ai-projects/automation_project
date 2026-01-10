// projects/products/core/engine/src/const_values.rs
use std::time::Duration;

pub const WS_MAX_MESSAGE_BYTES: usize = 256 * 1024; // 256 KiB
pub const WS_PING_EVERY: Duration = Duration::from_secs(20);
pub const WS_IDLE_TIMEOUT: Duration = Duration::from_secs(60);
pub const LOGIN_MAX_DURATION_MS: u64 = 7 * 24 * 60 * 60 * 1000; // 7 jours max
pub const DEFAULT_DURATION_MS: u64 = 60 * 60 * 1000; // 1 heure
