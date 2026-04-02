use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::store::in_flight_guard::InFlightGuard;

#[test]
fn drop_decrements_counter_by_guard_count() {
    let counter = Arc::new(AtomicUsize::new(7));

    {
        let guard = InFlightGuard::new(counter.clone(), 3);
        let current = counter.load(Ordering::Relaxed);
        assert_eq!(current, 7);
        std::hint::black_box(guard);
    }

    let current = counter.load(Ordering::Relaxed);
    assert_eq!(current, 4);
}
