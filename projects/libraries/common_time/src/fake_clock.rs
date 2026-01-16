//! Ce module fournit une implémentation de `Clock` destinée uniquement aux tests.
//! Il ne doit pas être utilisé dans le code de production.

use std::sync::{Arc, Mutex};

use crate::{Clock, MonoInstant, TimeSpan};

#[derive(Clone, Debug)]
pub struct FakeClock {
    state: Arc<Mutex<State>>,
}

#[derive(Debug)]
struct State {
    now: std::time::Instant,
}

impl FakeClock {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(State {
                now: std::time::Instant::now(),
            })),
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
