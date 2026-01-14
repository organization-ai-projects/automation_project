// projects/libraries/common_json/src/access.rs

use crate::error::{JsonError, JsonResult};
use crate::json_access::JsonAccess;
use crate::json_access_mut::JsonAccessMut;
use crate::value::{Json, JsonArray, JsonObject};

impl JsonAccess for Json {
    fn get_field(&self, key: &str) -> JsonResult<&Json> {
        match self {
            Json::Object(map) => map.get(key).ok_or_else(|| JsonError::missing_field(key)),
            _ => Err(JsonError::type_mismatch("object", self.type_name())),
        }
    }

    fn get_index(&self, index: usize) -> JsonResult<&Json> {
        match self {
            Json::Array(arr) => arr
                .get(index)
                .ok_or_else(|| JsonError::index_out_of_bounds(index, arr.len())),
            _ => Err(JsonError::type_mismatch("array", self.type_name())),
        }
    }

    fn get_path(&self, path: &str) -> JsonResult<&Json> {
        let mut current = self;

        for segment in path.split('.') {
            if segment.is_empty() {
                continue;
            }

            // Check if it's an array index [n]
            if let Some(idx_str) = segment.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                let index: usize = idx_str.parse().map_err(|_| JsonError::invalid_path(path))?;
                current = current.get_index(index)?;
            } else if let Some((field, rest)) = segment.split_once('[') {
                // Handle field[index] syntax
                current = current.get_field(field)?;
                let idx_str = rest
                    .strip_suffix(']')
                    .ok_or_else(|| JsonError::invalid_path(path))?;
                let index: usize = idx_str.parse().map_err(|_| JsonError::invalid_path(path))?;
                current = current.get_index(index)?;
            } else {
                current = current.get_field(segment)?;
            }
        }

        Ok(current)
    }

    fn as_str_strict(&self) -> JsonResult<&str> {
        self.as_str()
            .ok_or_else(|| JsonError::type_mismatch("string", self.type_name()))
    }

    fn as_i64_strict(&self) -> JsonResult<i64> {
        self.as_i64()
            .ok_or_else(|| JsonError::type_mismatch("i64", self.type_name()))
    }

    fn as_u64_strict(&self) -> JsonResult<u64> {
        self.as_u64()
            .ok_or_else(|| JsonError::type_mismatch("u64", self.type_name()))
    }

    fn as_f64_strict(&self) -> JsonResult<f64> {
        self.as_f64()
            .ok_or_else(|| JsonError::type_mismatch("f64", self.type_name()))
    }

    fn as_bool_strict(&self) -> JsonResult<bool> {
        self.as_bool()
            .ok_or_else(|| JsonError::type_mismatch("bool", self.type_name()))
    }

    fn as_array_strict(&self) -> JsonResult<&JsonArray> {
        self.as_array()
            .ok_or_else(|| JsonError::type_mismatch("array", self.type_name()))
    }

    fn as_object_strict(&self) -> JsonResult<&JsonObject> {
        self.as_object()
            .ok_or_else(|| JsonError::type_mismatch("object", self.type_name()))
    }

    fn type_name(&self) -> &'static str {
        match self {
            Json::Null => "null",
            Json::Bool(_) => "bool",
            Json::Number(_) => "number",
            Json::String(_) => "string",
            Json::Array(_) => "array",
            Json::Object(_) => "object",
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Json::Null => false,
            Json::Bool(b) => *b,
            Json::Number(n) => n.as_f64() != 0.0,
            Json::String(s) => !s.is_empty(),
            Json::Array(a) => !a.is_empty(),
            Json::Object(o) => !o.is_empty(),
        }
    }
}

impl JsonAccessMut for Json {
    fn get_field_mut(&mut self, key: &str) -> JsonResult<&mut Json> {
        match self {
            Json::Object(map) => map
                .get_mut(key)
                .ok_or_else(|| JsonError::missing_field(key)),
            _ => Err(JsonError::type_mismatch("object", self.type_name())),
        }
    }

    fn get_index_mut(&mut self, index: usize) -> JsonResult<&mut Json> {
        match self {
            Json::Array(arr) => {
                let len = arr.len();
                arr.get_mut(index)
                    .ok_or_else(|| JsonError::index_out_of_bounds(index, len))
            }
            _ => Err(JsonError::type_mismatch("array", self.type_name())),
        }
    }

    fn set_field<V: Into<Json>>(&mut self, key: &str, value: V) -> JsonResult<()> {
        match self {
            Json::Object(map) => {
                map.insert(key.to_string(), value.into());
                Ok(())
            }
            _ => Err(JsonError::type_mismatch("object", self.type_name())),
        }
    }

    fn remove_field(&mut self, key: &str) -> JsonResult<Option<Json>> {
        match self {
            Json::Object(map) => Ok(map.remove(key)),
            _ => Err(JsonError::type_mismatch("object", self.type_name())),
        }
    }

    fn push<V: Into<Json>>(&mut self, value: V) -> JsonResult<()> {
        match self {
            Json::Array(arr) => {
                arr.push(value.into());
                Ok(())
            }
            _ => Err(JsonError::type_mismatch("array", self.type_name())),
        }
    }

    fn insert_at<V: Into<Json>>(&mut self, index: usize, value: V) -> JsonResult<()> {
        match self {
            Json::Array(arr) => {
                if index > arr.len() {
                    return Err(JsonError::index_out_of_bounds(index, arr.len()));
                }
                arr.insert(index, value.into());
                Ok(())
            }
            _ => Err(JsonError::type_mismatch("array", self.type_name())),
        }
    }

    fn remove_at(&mut self, index: usize) -> JsonResult<Json> {
        match self {
            Json::Array(arr) => {
                if index >= arr.len() {
                    return Err(JsonError::index_out_of_bounds(index, arr.len()));
                }
                Ok(arr.remove(index))
            }
            _ => Err(JsonError::type_mismatch("array", self.type_name())),
        }
    }
}
