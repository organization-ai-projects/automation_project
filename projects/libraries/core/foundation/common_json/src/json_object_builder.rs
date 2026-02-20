// projects/libraries/common_json/src/json_object_builder.rs
use crate::{Json, JsonMap};

/// Fluent builder for constructing JSON objects.
///
/// Allows building JSON objects in a readable and type-safe manner.
pub struct JsonObjectBuilder {
    map: JsonMap,
}

impl JsonObjectBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self {
            map: JsonMap::new(),
        }
    }

    /// Adds a field to the object.
    ///
    /// The key and value are converted via `Into<String>` and `Into<Json>`.
    pub fn field<K: Into<String>, V: Into<Json>>(mut self, key: K, value: V) -> Self {
        self.map.insert(key.into(), value.into());
        self
    }

    /// Adds a field only if the value is `Some`.
    ///
    /// If `None`, the field is not added.
    pub fn field_opt<K: Into<String>, V: Into<Json>>(mut self, key: K, value: Option<V>) -> Self {
        if let Some(v) = value {
            self.map.insert(key.into(), v.into());
        }
        self
    }

    /// Adds a field only if the condition is true.
    pub fn field_if<K: Into<String>, V: Into<Json>>(
        self,
        condition: bool,
        key: K,
        value: V,
    ) -> Self {
        if condition {
            self.field(key, value)
        } else {
            self
        }
    }

    /// Finalizes and returns the JSON object.
    pub fn build(self) -> Json {
        Json::Object(self.map)
    }
}

impl Default for JsonObjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}
