use std::collections::BTreeMap;

use crate::diagnostics::error::Error;
use crate::model::doc_id::DocId;

use super::doc_snapshot::DocSnapshot;

pub struct DocStore {
    snapshots: BTreeMap<DocId, DocSnapshot>,
}

impl DocStore {
    pub fn new() -> Self {
        Self {
            snapshots: BTreeMap::new(),
        }
    }

    pub fn save(&mut self, snapshot: DocSnapshot) {
        self.snapshots.insert(snapshot.doc_id.clone(), snapshot);
    }

    pub fn load(&self, id: &DocId) -> Option<&DocSnapshot> {
        self.snapshots.get(id)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Error> {
        let mut serializable: BTreeMap<String, DocSnapshot> = BTreeMap::new();
        for (doc_id, snapshot) in &self.snapshots {
            serializable.insert(doc_id.0.clone(), snapshot.clone());
        }
        let json = common_json::to_string_pretty(&serializable)
            .map_err(|e| Error::Serialization(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| Error::Io(e.to_string()))
    }

    pub fn load_from_file(path: &str) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path).map_err(|e| Error::Io(e.to_string()))?;
        let loaded: BTreeMap<String, DocSnapshot> = common_json::from_json_str(&content)
            .map_err(|e| Error::Serialization(e.to_string()))?;
        let mut snapshots: BTreeMap<DocId, DocSnapshot> = BTreeMap::new();
        for (doc_id, mut snapshot) in loaded {
            snapshot.verify()?;
            snapshot.doc_id = DocId::new(doc_id.clone());
            snapshots.insert(DocId::new(doc_id), snapshot);
        }
        Ok(Self { snapshots })
    }

    pub fn doc_ids(&self) -> Vec<DocId> {
        self.snapshots.keys().cloned().collect()
    }
}

impl Default for DocStore {
    fn default() -> Self {
        Self::new()
    }
}
