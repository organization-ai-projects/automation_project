//! projects/products/stable/accounts/backend/src/store/in_flight_guard.rs
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct InFlightGuard {
    counter: Arc<AtomicUsize>,
    count: usize,
}

impl InFlightGuard {
    pub fn new(counter: Arc<AtomicUsize>, count: usize) -> Self {
        Self { counter, count }
    }
}

impl Drop for InFlightGuard {
    fn drop(&mut self) {
        self.counter.fetch_sub(self.count, Ordering::Relaxed);
    }
}
