use crate::{MonoInstant, TimeSpan};

pub trait Clock: Send + Sync {
    fn now(&self) -> MonoInstant;

    /// Blocking sleep (std-thread style).
    fn sleep(&self, span: TimeSpan);

    fn deadline_from_now(&self, span: TimeSpan) -> MonoInstant {
        let now = self.now().into_std();
        MonoInstant::from_std(now + span.as_duration())
    }
}
