use crate::{Clock, MonoInstant, TimeSpan};

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> MonoInstant {
        MonoInstant::now()
    }

    fn sleep(&self, span: TimeSpan) {
        std::thread::sleep(span.as_duration());
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        SystemClock
    }
}
