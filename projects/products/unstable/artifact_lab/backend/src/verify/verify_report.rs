#[derive(Debug)]
pub struct VerifyReport {
    pub ok: bool,
    pub entry_count: usize,
    pub results: Vec<EntryResult>,
}

#[derive(Debug)]
pub struct EntryResult {
    pub path: String,
    pub status: EntryStatus,
}

#[derive(Debug)]
pub enum EntryStatus {
    Ok,
    HashMismatch { expected: String, actual: String },
    SizeMismatch { expected: u64, actual: u64 },
}

impl EntryResult {
    pub fn status_label(&self) -> &'static str {
        match &self.status {
            EntryStatus::Ok => "ok",
            EntryStatus::HashMismatch { .. } => "hash_mismatch",
            EntryStatus::SizeMismatch { .. } => "size_mismatch",
        }
    }
}
