// projects/libraries/protocol/src/metadata.rs
use crate::protocol_id::ProtocolId;
use crate::validation_error::ValidationError;
use common::Id128;
use common_time::timestamp_utils::Timestamp;
use common_time::timestamp_utils::current_timestamp_ms;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

/// Maximum acceptable timestamp drift into the future (1 hour in milliseconds)
const MAX_FUTURE_DRIFT_MS: u64 = 3600 * 1000;

/// Metadata associated with commands and events
///
/// Contains timing and identification information for protocol messages
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    pub request_id: ProtocolId,
    pub job_id: Option<ProtocolId>,

    // Routing (engine uses this)
    pub product_id: Option<ProtocolId>,

    // Audit / observability
    pub client_id: Option<ProtocolId>,
    pub timestamp_ms: Option<Timestamp>,

    // Compatibility
    pub schema_version: Option<u32>,
}

impl Metadata {
    /// Creates new metadata with the current timestamp and a generated ID
    ///
    /// The ID is derived from the current timestamp and a monotonic counter,
    /// ensuring uniqueness for IDs generated within this process.
    pub fn now() -> Self {
        let timestamp_ms = Self::current_timestamp_ms();
        let request_id = Self::generate_request_id(timestamp_ms);
        Self {
            timestamp_ms: Some(timestamp_ms),
            request_id,
            ..Default::default()
        }
    }

    /// Creates metadata with a specific timestamp and generated ID
    pub fn with_timestamp(timestamp_ms: u64) -> Self {
        let request_id = Self::generate_request_id(timestamp_ms);
        Self {
            timestamp_ms: Some(timestamp_ms),
            request_id,
            ..Default::default()
        }
    }

    /// Creates metadata with specific timestamp and ProtocolId.
    pub fn new_with_protocol_id(timestamp_ms: u64, request_id: ProtocolId) -> Self {
        Self {
            timestamp_ms: Some(timestamp_ms),
            request_id,
            ..Default::default()
        }
    }

    /// Creates metadata with specific timestamp and ID string.
    /// Returns an error when the request_id is not a valid hex ProtocolId.
    pub fn try_new(
        timestamp_ms: u64,
        request_id: &str,
    ) -> Result<Self, <ProtocolId as FromStr>::Err> {
        let request_id = ProtocolId::from_str(request_id)?;
        Ok(Self::new_with_protocol_id(timestamp_ms, request_id))
    }

    /// Gets the current system timestamp in milliseconds since UNIX epoch
    pub fn current_timestamp_ms() -> u64 {
        current_timestamp_ms()
    }

    /// Validates the metadata
    ///
    /// Checks that the timestamp is reasonable (not too far in the future)
    pub fn validate(&self) -> Result<(), ValidationError> {
        let now = Self::current_timestamp_ms();

        if let Some(timestamp_ms) = self.timestamp_ms {
            // Check if timestamp is too far in the future
            if timestamp_ms > now + MAX_FUTURE_DRIFT_MS {
                return Err(ValidationError::InvalidTimestamp(format!(
                    "Timestamp {} is too far in the future (current: {})",
                    timestamp_ms, now
                )));
            }
        }

        Ok(())
    }

    /// Generates a unique request ID based on timestamp and a monotonic counter.
    fn generate_request_id(timestamp: u64) -> ProtocolId {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let counter = COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&timestamp.to_be_bytes());
        bytes[8..16].copy_from_slice(&counter.to_be_bytes());
        ProtocolId::new(Id128::from_bytes_unchecked(bytes))
    }

    /// Converts timestamp to a human-readable ISO 8601 string (UTC)
    ///
    /// Note: This is a basic implementation. For production use, consider using
    /// a dedicated datetime library like `chrono`.
    pub fn timestamp_to_string(&self) -> String {
        if let Some(timestamp_ms) = self.timestamp_ms {
            let secs = timestamp_ms / 1000;
            let millis = timestamp_ms % 1000;
            format!("{}+{:03}ms", secs, millis)
        } else {
            "Unknown timestamp".to_string()
        }
    }

    /// Convert Metadata to a unique String representation
    pub fn to_key(&self) -> String {
        self.request_id.to_string()
    }
}

impl Hash for Metadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.request_id.hash(state);
        self.job_id.hash(state);
        self.product_id.hash(state);
        self.client_id.hash(state);
        self.timestamp_ms.hash(state);
        self.schema_version.hash(state);
    }
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Self) -> bool {
        self.request_id == other.request_id
            && self.job_id == other.job_id
            && self.product_id == other.product_id
            && self.client_id == other.client_id
            && self.timestamp_ms == other.timestamp_ms
            && self.schema_version == other.schema_version
    }
}

impl Eq for Metadata {}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Metadata {{ request_id: {}, job_id: {:?}, product_id: {:?}, client_id: {:?}, timestamp_ms: {:?}, schema_version: {:?} }}",
            self.request_id,
            self.job_id,
            self.product_id,
            self.client_id,
            self.timestamp_ms,
            self.schema_version
        )
    }
}
