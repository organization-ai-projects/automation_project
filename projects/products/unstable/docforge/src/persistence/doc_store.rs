use std::collections::BTreeMap;

use crate::diagnostics::error::DocError;
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

    pub fn save_to_file(&self, path: &str) -> Result<(), DocError> {
        let json = serde_json::to_string_pretty(&self.snapshots)
            .map_err(|e| DocError::Serialization(e.to_string()))?;
        std::fs::write(path, json).map_err(|e| DocError::Io(e.to_string()))
    }

    pub fn load_from_file(path: &str) -> Result<Self, DocError> {
        let content = std::fs::read_to_string(path).map_err(|e| DocError::Io(e.to_string()))?;
        let snapshots: BTreeMap<DocId, DocSnapshot> =
            serde_json::from_str(&content).map_err(|e| DocError::Serialization(e.to_string()))?;
        Ok(Self { snapshots })
    }
}

impl Default for DocStore {
    fn default() -> Self {
        Self::new()
    }
}
