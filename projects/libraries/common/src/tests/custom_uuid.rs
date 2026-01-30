#[cfg(test)]
mod tests {
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
        let set = Arc::new(Mutex::new(HashSet::new()));
        let mut handles = vec![];

        for _ in 0..8 {
            let set = set.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..50_000 {
                    let id = Id128::new(1, None, None).to_string();
                    let mut s = set
                        .lock()
                        .expect("Failed to acquire lock on set in multithreaded test");
                    if !s.insert(id) {
                        panic!("duplicate");
                    }
                }
            }));
        }

        for h in handles {
            h.join().expect("Thread join failed in multithreaded test");
        }
    }
}
