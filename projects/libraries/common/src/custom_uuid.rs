// projects/libraries/common/src/custom_uuid.rs
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use std::str::FromStr;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id128([u8; 16]);

#[derive(Debug)]
pub enum IdError {
    InvalidLen,
    InvalidHex,
}

impl Id128 {
    /// Create a new generator-friendly ID with optional boot_id and process_id.
    pub fn new(node_id: u16, boot_id: Option<u16>, process_id: Option<u16>) -> Self {
        let boot_id = boot_id.unwrap_or_else(random_u16);
        let process_id = process_id.unwrap_or_else(random_u16);
        Self::new_with_params(node_id, boot_id, process_id)
    }

    /// Create a new generator-friendly ID with explicit parameters.
    ///
    /// node_id: stable per machine/agent cluster (must be unique-ish).
    /// boot_id: changes each program start (or per run_id seed).
    /// process_id: per-process id (pid truncated or random).
    fn new_with_params(node_id: u16, boot_id: u16, process_id: u16) -> Self {
        let ms = Self::monotonic_ms();
        let seq = Self::next_seq_for_ms(ms);

        let mut id = [0u8; 16];

        // Timestamp, node_id, process_id, boot_id, and sequence are combined to form the ID.
        id[0..6].copy_from_slice(&ms.to_be_bytes()[2..8]);
        id[6..8].copy_from_slice(&node_id.to_be_bytes());
        id[8..10].copy_from_slice(&process_id.to_be_bytes());
        id[10..12].copy_from_slice(&boot_id.to_be_bytes());
        id[12..16].copy_from_slice(&seq.to_be_bytes());

        Self(id)
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    pub fn from_bytes_unchecked(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub fn try_from_bytes(bytes: [u8; 16]) -> Result<Self, IdError> {
        if bytes == [0u8; 16] {
            return Err(IdError::InvalidHex);
        }
        Ok(Self(bytes))
    }

    // 48-bit ms timestamp stored big-endian in first 6 bytes.
    // Global monotonic time guard (process-wide).
    fn static_last_ms() -> &'static AtomicU64 {
        static LAST_MS: AtomicU64 = AtomicU64::new(0);
        &LAST_MS
    }

    /// Converts the ID to a 32-character lowercase hexadecimal string.
    pub fn to_hex(&self) -> String {
        let mut s = String::with_capacity(32);
        for byte in &self.0 {
            use std::fmt::Write;
            let _ = write!(s, "{:02x}", byte);
        }
        s
    }

    pub fn from_hex(s: &str) -> Result<Self, IdError> {
        if s.len() != 32 {
            return Err(IdError::InvalidLen);
        }
        let mut out = [0u8; 16];
        let bytes = s.as_bytes();

        for i in 0..16 {
            let hi = decode_hex_nibble(bytes[i * 2]).ok_or(IdError::InvalidHex)?;
            let lo = decode_hex_nibble(bytes[i * 2 + 1]).ok_or(IdError::InvalidHex)?;
            out[i] = (hi << 4) | lo;
        }

        Ok(Self(out))
    }

    /// Extracts the 48-bit timestamp from the ID.
    pub fn timestamp_ms(&self) -> u64 {
        let mut ms_be = [0u8; 8];
        ms_be[2..8].copy_from_slice(&self.0[0..6]);
        u64::from_be_bytes(ms_be)
    }

    pub fn node_id(&self) -> u16 {
        u16::from_be_bytes([self.0[6], self.0[7]])
    }

    pub fn process_id(&self) -> u16 {
        u16::from_be_bytes([self.0[8], self.0[9]])
    }

    pub fn boot_id(&self) -> u16 {
        u16::from_be_bytes([self.0[10], self.0[11]])
    }

    pub fn seq(&self) -> u32 {
        u32::from_be_bytes([self.0[12], self.0[13], self.0[14], self.0[15]])
    }

    fn now_ms() -> u64 {
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_millis(0));
        dur.as_millis() as u64
    }

    /// Generates a monotonic timestamp in milliseconds, ensuring no rollback within the process.
    fn monotonic_ms() -> u64 {
        let now = Self::now_ms();
        let last = Self::static_last_ms();
        let mut cur = last.load(Ordering::Relaxed);
        loop {
            let chosen = now.max(cur);
            match last.compare_exchange(cur, chosen, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => return chosen,
                Err(v) => cur = v,
            }
        }
    }

    /// Generates the next sequence number for the given timestamp.
    fn next_seq_for_ms(ms: u64) -> u32 {
        static STATE: Mutex<(u64, u32)> = Mutex::new((0, 0));
        let mut state = STATE.lock().unwrap();

        if state.0 != ms {
            state.0 = ms;
            state.1 = random_u32();
        } else {
            state.1 = state.1.wrapping_add(1);
        }

        state.1
    }

    pub fn load_or_create_node_id(path: &Path) -> u16 {
        use std::fs::{self, OpenOptions};
        use std::io::{Read, Write};

        // 1) Try read existing
        if let Ok(mut file) = fs::File::open(path) {
            let mut buf = [0u8; 2];
            if file.read_exact(&mut buf).is_ok() {
                return u16::from_be_bytes(buf);
            }
        }

        // 2) Generate
        let id = random_u16();

        // 3) Ensure parent dir exists
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // 4) Try create_new to avoid races (best effort)
        if let Ok(mut file) = OpenOptions::new().write(true).create_new(true).open(path) {
            let _ = file.write_all(&id.to_be_bytes());
            return id;
        }

        // 5) Someone else created it. Read again.
        if let Ok(mut file) = fs::File::open(path) {
            let mut buf = [0u8; 2];
            if file.read_exact(&mut buf).is_ok() {
                return u16::from_be_bytes(buf);
            }
        }

        // 6) Last resort: return our generated id (should be extremely rare)
        id
    }
}

impl fmt::Display for Id128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl FromStr for Id128 {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl std::error::Error for IdError {}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdError::InvalidLen => write!(f, "Invalid length for ID"),
            IdError::InvalidHex => write!(f, "Invalid hex format for ID"),
        }
    }
}

fn decode_hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn random_u16() -> u16 {
    let mut b = [0u8; 2];
    OsRng.fill_bytes(&mut b);
    u16::from_be_bytes(b)
}

fn random_u32() -> u32 {
    let mut b = [0u8; 4];
    OsRng.fill_bytes(&mut b);
    u32::from_be_bytes(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn roundtrip_hex() {
        let id = Id128::new(42, None, None);
        let s = id.to_string();
        let parsed: Id128 = s.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn ordering_is_mostly_time_sorted() {
        let a = Id128::new(1, None, None);
        let b = Id128::new(1, None, None);
        assert!(a.timestamp_ms() <= b.timestamp_ms());
    }

    #[test]
    fn fields_extract() {
        let node = 7u16;
        let boot = 9u16;
        let proc_ = 11u16;
        let id = Id128::new_with_params(node, boot, proc_);
        assert_eq!(id.node_id(), node);
        assert_eq!(id.boot_id(), boot);
        assert_eq!(id.process_id(), proc_);
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
                    let mut s = set.lock().unwrap();
                    if !s.insert(id) {
                        panic!("duplicate");
                    }
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}
