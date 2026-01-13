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
        let secs = self.inner.as_secs().saturating_mul(n);
        Self::from_secs(secs)
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
