// projects/libraries/protocol/src/metadata.rs
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::ValidationError;

/// Maximum acceptable timestamp drift into the future (1 hour in milliseconds)
const MAX_FUTURE_DRIFT_MS: u64 = 3600 * 1000;

/// Metadata associated with commands and events
///
/// Contains timing and identification information for protocol messages
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Metadata {
    /// Timestamp in milliseconds since UNIX epoch (January 1, 1970 UTC)
    pub timestamp: u64,
    /// Unique identifier for the message
    pub id: u64,
}

impl Metadata {
    /// Creates new metadata with the current timestamp and a generated ID
    ///
    /// The ID is generated from the current timestamp combined with process randomness
    pub fn now() -> Self {
        let timestamp = Self::current_timestamp_ms();
        let id = Self::generate_id(timestamp);
        Self { timestamp, id }
    }

    /// Creates metadata with a specific timestamp and generated ID
    pub fn with_timestamp(timestamp: u64) -> Self {
        let id = Self::generate_id(timestamp);
        Self { timestamp, id }
    }

    /// Creates metadata with specific timestamp and ID
    pub fn new(timestamp: u64, id: u64) -> Self {
        Self { timestamp, id }
    }

    /// Gets the current system timestamp in milliseconds since UNIX epoch
    pub fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch")
            .as_millis() as u64
    }

    /// Validates the metadata
    ///
    /// Checks that the timestamp is reasonable (not too far in the future)
    pub fn validate(&self) -> Result<(), ValidationError> {
        let now = Self::current_timestamp_ms();

        // Check if timestamp is too far in the future
        if self.timestamp > now + MAX_FUTURE_DRIFT_MS {
            return Err(ValidationError::InvalidTimestamp(
                format!("Timestamp {} is too far in the future (current: {})", self.timestamp, now)
            ));
        }

        Ok(())
    }

    /// Generates a unique ID based on timestamp
    ///
    /// Combines timestamp with a simple counter for uniqueness
    fn generate_id(timestamp: u64) -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        // Combine timestamp (upper 42 bits) with counter (lower 22 bits)
        ((timestamp & 0x3FFFFFFFFFF) << 22) | (counter & 0x3FFFFF)
    }

    /// Converts timestamp to a human-readable ISO 8601 string (UTC)
    ///
    /// Note: This is a basic implementation. For production use, consider using
    /// a dedicated datetime library like `chrono`.
    pub fn timestamp_to_string(&self) -> String {
        let secs = self.timestamp / 1000;
        let millis = self.timestamp % 1000;
        format!("{}+{:03}ms", secs, millis)
    }
}
