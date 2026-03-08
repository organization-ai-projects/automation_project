use crate::storage::record::Record;
use crate::storage::record_id::RecordId;
use common_json::Json;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct RecordStore {
    next_id: u64,
    records: BTreeMap<RecordId, Record>,
}

impl RecordStore {
    pub fn insert(&mut self, data: Json) -> RecordId {
        self.next_id = self.next_id.saturating_add(1);
        let id = RecordId(self.next_id);
        self.records.insert(id, Record { id, data });
        id
    }

    pub fn update(&mut self, id: u64, data: Json) -> bool {
        let id = RecordId(id);
        if let Some(record) = self.records.get_mut(&id) {
            record.data = data;
            true
        } else {
            false
        }
    }

    pub fn delete(&mut self, id: u64) -> bool {
        self.records.remove(&RecordId(id)).is_some()
    }

    pub fn get(&self, id: u64) -> Option<&Record> {
        self.records.get(&RecordId(id))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Record> {
        self.records.values()
    }
}
