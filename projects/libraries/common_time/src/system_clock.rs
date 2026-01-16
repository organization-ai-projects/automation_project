use crate::{Clock, MonoInstant, TimeSpan};
use chrono::{DateTime, Utc};

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

impl SystemClock {
    pub fn now_rfc3339() -> String {
        let now: DateTime<Utc> = Utc::now();
        now.to_rfc3339()
    }
}
