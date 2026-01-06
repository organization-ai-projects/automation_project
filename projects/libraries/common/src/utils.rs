// Example: A utility function
pub fn is_valid_id(id: u64) -> bool {
    id > 0
}

// Validate if a string is a valid name
pub fn is_valid_name(name: &str) -> bool {
    !name.trim().is_empty()
}

// Format a timestamp as a readable string
pub fn format_timestamp(timestamp: u64) -> String {
    use chrono::{TimeZone, Utc};
    let datetime = Utc
        .timestamp_opt(timestamp as i64, 0)
        .single()
        .unwrap_or_else(Utc::now);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

// Durées standard pour des intervalles de temps
pub mod time_durations {
    pub const FIFTEEN_MINUTES: u64 = 15 * 60;
    pub const ONE_HOUR: u64 = 60 * 60;
    pub const ONE_DAY: u64 = 24 * 60 * 60;
    pub const ONE_WEEK: u64 = 7 * 24 * 60 * 60;
    pub const ONE_MONTH: u64 = 30 * 24 * 60 * 60;
}

pub use time_durations::*;
