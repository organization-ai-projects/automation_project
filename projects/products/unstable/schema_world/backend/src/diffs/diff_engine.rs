use crate::diffs::diff::Diff;
use crate::snapshots::snapshot::Snapshot;
use crate::storage::record_id::RecordId;
use common_json::Json;
use std::collections::{BTreeMap, HashMap};

pub struct DiffEngine;

impl DiffEngine {
    pub fn diff(from: &Snapshot, to: &Snapshot) -> Json {
        let from_map = to_map(from);
        let to_map = to_map(to);

        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut updated = Vec::new();

        for (id, record) in &to_map {
            match from_map.get(id) {
                None => added.push(object_with_record(*id, record.clone())),
                Some(previous) if previous != record => {
                    updated.push(object_with_update(*id, previous.clone(), record.clone()))
                }
                _ => {}
            }
        }

        for (id, record) in &from_map {
            if !to_map.contains_key(id) {
                removed.push(object_with_record(*id, record.clone()));
            }
        }

        common_json::to_value(&Diff {
            added,
            removed,
            updated,
        })
        .unwrap_or_else(|_| {
            let mut root = HashMap::new();
            root.insert("added".to_string(), Json::Array(Vec::new()));
            root.insert("removed".to_string(), Json::Array(Vec::new()));
            root.insert("updated".to_string(), Json::Array(Vec::new()));
            Json::Object(root)
        })
    }
}

fn to_map(snapshot: &Snapshot) -> BTreeMap<RecordId, Json> {
    snapshot
        .records
        .iter()
        .map(|record| (record.id, record.data.clone()))
        .collect()
}

fn object_with_record(id: RecordId, record: Json) -> Json {
    let mut map = HashMap::new();
    map.insert("id".to_string(), Json::from(id.0));
    map.insert("record".to_string(), record);
    Json::Object(map)
}

fn object_with_update(id: RecordId, from: Json, to: Json) -> Json {
    let mut map = HashMap::new();
    map.insert("id".to_string(), Json::from(id.0));
    map.insert("from".to_string(), from);
    map.insert("to".to_string(), to);
    Json::Object(map)
}
