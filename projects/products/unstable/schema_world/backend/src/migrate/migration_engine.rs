use crate::migrate::migration::Migration;
use common_json::Json;
use std::collections::HashMap;

pub struct MigrationEngine;

impl MigrationEngine {
    pub fn apply_forward(record: &Json, migration: &Migration) -> Result<Json, String> {
        let mut object = record
            .as_object()
            .cloned()
            .ok_or_else(|| "migration requires object record".to_string())?;

        for (from, to) in &migration.renames {
            if let Some(value) = object.remove(from) {
                object.insert(to.clone(), value);
            }
        }

        for (field, value) in &migration.defaults {
            object.entry(field.clone()).or_insert_with(|| value.clone());
        }

        Ok(Json::Object(sort_object(object)))
    }

    pub fn apply_reverse(record: &Json, migration: &Migration) -> Result<Json, String> {
        let mut object = record
            .as_object()
            .cloned()
            .ok_or_else(|| "reverse migration requires object record".to_string())?;

        for to in migration.defaults.keys() {
            object.remove(to);
        }

        for (from, to) in &migration.renames {
            if let Some(value) = object.remove(to) {
                object.insert(from.clone(), value);
            }
        }

        Ok(Json::Object(sort_object(object)))
    }
}

fn sort_object(map: HashMap<String, Json>) -> HashMap<String, Json> {
    let mut keys: Vec<String> = map.keys().cloned().collect();
    keys.sort();

    let mut sorted = HashMap::new();
    for key in keys {
        if let Some(value) = map.get(&key) {
            sorted.insert(key, value.clone());
        }
    }
    sorted
}
