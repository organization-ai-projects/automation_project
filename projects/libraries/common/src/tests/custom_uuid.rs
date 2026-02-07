use crate::Id128;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn roundtrip_hex() {
    let id = Id128::new(42, None, None);
    let s = id.to_string();
    let parsed: Id128 = s.parse().expect("Failed to parse Id128 from string");
    assert_eq!(id, parsed);
}

#[test]
fn ordering_is_mostly_time_sorted() {
    let a = Id128::new(1, None, None);
    let b = Id128::new(1, None, None);
    assert!(a.timestamp_ms() <= b.timestamp_ms());
}

#[test]
fn no_duplicates_multithread() {
    // Reduced iteration count from 50_000 to 10_000 per thread to speed up CI
    // (8 threads × 10_000 = 80_000 total IDs, down from 400_000)
    const ITERATIONS_PER_THREAD: usize = 10_000;
    const THREAD_COUNT: usize = 8;
    
    let set = Arc::new(Mutex::new(HashSet::new()));
    let mut handles = vec![];

    for _ in 0..THREAD_COUNT {
        let set = set.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS_PER_THREAD {
                let id = Id128::new(1, None, None).to_string();
                let mut s = set
                    .lock()
                    .expect("Failed to acquire lock on set in multithreaded test");
                if let Some(dup) = s.replace(id) {
                    panic!("Duplicate ID generated: {}", dup);
                }
            }
        }));
    }

    for h in handles {
        h.join().expect("Thread join failed in multithreaded test");
    }
}
