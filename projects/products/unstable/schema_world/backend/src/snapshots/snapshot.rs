use crate::storage::record::Record;
use crate::storage::record_store::RecordStore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub schema_version: u32,
    pub records: Vec<Record>,
}

impl Snapshot {
    pub fn from_store(schema_version: u32, store: &RecordStore) -> Self {
        let records: Vec<Record> = store.iter().cloned().collect();
        Self {
            schema_version,
            records,
        }
    }
}
