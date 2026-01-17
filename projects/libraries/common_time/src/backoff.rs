// projects/libraries/common_time/src/backoff.rs
use crate::{TimeSpan, units};

#[derive(Clone, Copy, Debug)]
pub struct Backoff {
    base: TimeSpan,
    max: TimeSpan,
}

impl Backoff {
    pub const fn new(base: TimeSpan, max: TimeSpan) -> Self {
        Self { base, max }
    }

    /// Simple exponential: base * 2^attempt, clamped to max.
    pub const fn for_attempt(self, attempt: u32) -> TimeSpan {
        let factor = if attempt < 64 { 1u64 << attempt } else { 0 };
        let scaled = self.base.saturating_mul(factor);
        if scaled.as_secs() > self.max.as_secs() {
            self.max
        } else {
            scaled
        }
    }
}

impl Default for Backoff {
    fn default() -> Self {
        Self::new(units::ONE_MINUTE, units::span(30, units::MIN))
    }
}
