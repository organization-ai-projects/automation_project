use chrono::{TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

/// Type alias pour représenter un timestamp en millisecondes.
pub type Timestamp = u64;

/// Retourne le timestamp actuel en millisecondes depuis l'époque UNIX.
pub fn current_timestamp_ms() -> Timestamp {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch")
        .as_millis() as Timestamp
}

/// Valide qu'un timestamp n'est pas trop loin dans le futur.
pub fn validate_timestamp(timestamp_ms: Timestamp, max_drift_ms: Timestamp) -> Result<(), String> {
    let now = current_timestamp_ms();
    if timestamp_ms > now + max_drift_ms {
        Err(format!(
            "Timestamp {} is too far in the future (current: {}, max drift: {})",
            timestamp_ms, now, max_drift_ms
        ))
    } else {
        Ok(())
    }
}

/// Formate un timestamp en une chaîne lisible.
pub fn format_timestamp(timestamp: Timestamp) -> String {
    let datetime = Utc
        .timestamp_opt(timestamp as i64, 0)
        .single()
        .unwrap_or_else(Utc::now);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
