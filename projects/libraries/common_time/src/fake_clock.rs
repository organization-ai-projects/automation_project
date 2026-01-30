//! This module provides an implementation of `Clock` intended for testing purposes only.
//! It should not be used in production code.
// projects/libraries/common_time/src/fake_clock.rs
use std::sync::{Arc, Mutex};

use crate::{Clock, MonoInstant, TimeSpan, fake_clock_state::FakeClockState};

#[derive(Clone, Debug)]
pub struct FakeClock {
    state: Arc<Mutex<FakeClockState>>,
}

impl FakeClock {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(FakeClockState::new())),
        }
    }

    /// Advance the fake time.
    pub fn advance(&self, span: TimeSpan) {
        let mut s = self.state.lock().expect("FakeClock poisoned");
        s.now += span.as_duration();
    }

    /// Set absolute now (rarely needed).
    pub fn set_now_std(&self, now: std::time::Instant) {
        let mut s = self.state.lock().expect("FakeClock poisoned");
        s.now = now;
    }
}

impl Clock for FakeClock {
    fn now(&self) -> MonoInstant {
        let s = self.state.lock().expect("FakeClock poisoned");
        MonoInstant::from_std(s.now)
    }

    fn sleep(&self, span: TimeSpan) {
        // In tests you typically call advance() yourself.
        self.advance(span);
    }
}

impl Default for FakeClock {
    fn default() -> Self {
        Self::new()
    }
}
