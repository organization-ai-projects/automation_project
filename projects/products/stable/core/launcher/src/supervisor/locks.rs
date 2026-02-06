// projects/products/stable/core/launcher/src/supervisor/locks.rs
use std::sync::{Arc, Mutex, MutexGuard};

use crate::logging::log_message;

type LockGuard<'a, T> = MutexGuard<'a, T>;

pub(crate) fn lock_recover<'a, T>(m: &'a Mutex<T>, what: &str) -> LockGuard<'a, T> {
    match m.lock() {
        Ok(g) => g,
        Err(p) => {
            log_message(
                &format!("{what} lock poisoned; recovering"),
                true,
                "launcher",
            );
            p.into_inner()
        }
    }
}

pub(crate) fn lock_recover_arc<'a, T>(m: &'a Arc<Mutex<T>>, what: &str) -> MutexGuard<'a, T> {
    match m.lock() {
        Ok(g) => g,
        Err(p) => {
            log_message(
                &format!("{what} lock poisoned; recovering"),
                true,
                "launcher",
            );
            p.into_inner()
        }
    }
}
