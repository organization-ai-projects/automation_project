// projects/libraries/common_time/src/units.rs
use crate::TimeSpan;

/// Units in seconds (u64) to construct TimeSpan cleanly.
pub const SEC: u64 = 1;
pub const MIN: u64 = 60 * SEC;
pub const HOUR: u64 = 60 * MIN;
pub const DAY: u64 = 24 * HOUR;
pub const WEEK: u64 = 7 * DAY;

/// Approximation by design (calendar ≠ duration)
pub const MONTH_APPROX: u64 = 30 * DAY;

/// Stable DSL (no op traits in const)
pub const fn span(n: u64, unit_secs: u64) -> TimeSpan {
    TimeSpan::from_secs(n.saturating_mul(unit_secs))
}

// Ready-to-use examples
pub const ONE_SECOND: TimeSpan = span(1, SEC);
pub const ONE_MINUTE: TimeSpan = span(1, MIN);
pub const FIFTEEN_MINUTES: TimeSpan = span(15, MIN);
pub const THIRTY_MINUTES: TimeSpan = span(30, MIN);
pub const FORTY_FIVE_MINUTES: TimeSpan = span(45, MIN);

pub const ONE_HOUR: TimeSpan = span(1, HOUR);
pub const ONE_DAY: TimeSpan = span(1, DAY);
pub const ONE_WEEK: TimeSpan = span(1, WEEK);
pub const ONE_MONTH: TimeSpan = span(1, MONTH_APPROX);
