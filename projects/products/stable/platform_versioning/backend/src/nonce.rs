use std::sync::atomic::{AtomicU32, Ordering};

pub fn next_nonce() -> u32 {
    static NONCE: AtomicU32 = AtomicU32::new(1);
    NONCE.fetch_add(1, Ordering::Relaxed)
}
