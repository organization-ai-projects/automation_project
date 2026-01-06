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
    use chrono::{NaiveDateTime, Utc};
    let naive = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
        .unwrap_or_else(|| Utc::now().naive_utc());
    naive.format("%Y-%m-%d %H:%M:%S").to_string()
}
