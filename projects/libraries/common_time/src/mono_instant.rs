use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonoInstant {
    inner: Instant,
}

impl MonoInstant {
    pub fn now() -> Self {
        Self {
            inner: Instant::now(),
        }
    }

    pub fn elapsed(self) -> std::time::Duration {
        self.inner.elapsed()
    }

    pub fn into_std(self) -> Instant {
        self.inner
    }

    pub fn from_std(i: Instant) -> Self {
        Self { inner: i }
    }
}
