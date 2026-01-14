// projects/libraries/common_time/src/time_span.rs
use std::fmt;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSpan {
    inner: Duration,
}

impl TimeSpan {
    pub const fn from_secs(secs: u64) -> Self {
        Self {
            inner: Duration::from_secs(secs),
        }
    }

    pub const fn from_millis(ms: u64) -> Self {
        Self {
            inner: Duration::from_millis(ms),
        }
    }

    pub const fn as_duration(self) -> Duration {
        self.inner
    }

    pub const fn as_secs(self) -> u64 {
        self.inner.as_secs()
    }

    pub const fn saturating_mul(self, n: u64) -> Self {
        let nanos = self.inner.as_nanos();
        let mul = nanos.saturating_mul(n as u128);
        let clamped = if mul > u64::MAX as u128 {
            u64::MAX
        } else {
            mul as u64
        };
        Self {
            inner: Duration::from_nanos(clamped),
        }
    }
}

impl From<Duration> for TimeSpan {
    fn from(d: Duration) -> Self {
        Self { inner: d }
    }
}

impl From<TimeSpan> for Duration {
    fn from(s: TimeSpan) -> Self {
        s.inner
    }
}

impl Default for TimeSpan {
    fn default() -> Self {
        TimeSpan::from_secs(30) // Default value of 30 seconds
    }
}

impl fmt::Display for TimeSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}s", self.as_secs())
    }
}
