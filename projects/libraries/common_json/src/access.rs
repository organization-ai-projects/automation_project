// projects/libraries/common_json/src/access.rs
use crate::Json;
use crate::json_access::JsonAccess;
use crate::json_access_mut::JsonAccessMut;
use crate::json_error::{JsonError, JsonResult};
use crate::json_error_code::JsonErrorCode;
use crate::value::{JsonArray, JsonObject};

impl JsonAccess for Json {
    fn get_field(&self, key: &str) -> JsonResult<&Json> {
        match self {
            Json::Object(map) => map
                .get(key)
                .ok_or_else(|| JsonError::new(JsonErrorCode::FieldNotFound)),
            _ => Err(JsonError::new(JsonErrorCode::TypeMismatch)),
        }
    }

    fn get_index(&self, index: usize) -> JsonResult<&Json> {
        match self {
            Json::Array(arr) => {
                let len = arr.len();
                arr.get(index).ok_or_else(|| {
                    JsonError::new(JsonErrorCode::IndexOutOfBounds).context(format!(
                        "Index {index} out of bounds for array of length {len}"
                    ))
                })
            }
            _ => Err(JsonError::new(JsonErrorCode::TypeMismatch)),
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
                let index: usize = idx_str
                    .parse()
                    .map_err(|_| JsonError::new(JsonErrorCode::InvalidPath))?;
                current = current.get_index(index)?;
            } else if let Some((field, rest)) = segment.split_once('[') {
                // Handle field[index] syntax
                current = current.get_field(field)?;
                let idx_str = rest
                    .strip_suffix(']')
                    .ok_or_else(|| JsonError::new(JsonErrorCode::InvalidPath))?;
                let index: usize = idx_str
                    .parse()
                    .map_err(|_| JsonError::new(JsonErrorCode::InvalidPath))?;
                current = current.get_index(index)?;
            } else {
                current = current.get_field(segment)?;
            }
        }

        Ok(current)
    }

    fn as_str_strict(&self) -> JsonResult<&str> {
        self.as_str()
            .ok_or_else(|| JsonError::new(JsonErrorCode::TypeMismatch))
    }

    fn as_i64_strict(&self) -> JsonResult<i64> {
        self.as_i64()
            .ok_or_else(|| JsonError::new(JsonErrorCode::TypeMismatch))
    }

    fn as_u64_strict(&self) -> JsonResult<u64> {
        self.as_u64()
            .ok_or_else(|| JsonError::new(JsonErrorCode::TypeMismatch))
    }

    fn as_f64_strict(&self) -> JsonResult<f64> {
        self.as_f64()
            .ok_or_else(|| JsonError::new(JsonErrorCode::TypeMismatch))
    }

    fn as_bool_strict(&self) -> JsonResult<bool> {
        self.as_bool()
            .ok_or_else(|| JsonError::new(JsonErrorCode::TypeMismatch))
    }

    fn as_array_strict(&self) -> JsonResult<&JsonArray> {
        self.as_array()
            .ok_or_else(|| JsonError::new(JsonErrorCode::TypeMismatch))
    }

    fn as_object_strict(&self) -> JsonResult<&JsonObject> {
        self.as_object()
            .ok_or_else(|| JsonError::new(JsonErrorCode::TypeMismatch))
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
                .ok_or_else(|| JsonError::new(JsonErrorCode::FieldNotFound)),
            _ => Err(JsonError::new(JsonErrorCode::TypeMismatch)),
        }
    }

    fn get_index_mut(&mut self, index: usize) -> JsonResult<&mut Json> {
        match self {
            Json::Array(arr) => {
                let len = arr.len();
                arr.get_mut(index).ok_or_else(|| {
                    JsonError::new(JsonErrorCode::IndexOutOfBounds).context(format!(
                        "Index {index} out of bounds for array of length {len}"
                    ))
                })
            }
            _ => Err(JsonError::new(JsonErrorCode::TypeMismatch)),
        }
    }

    fn set_field<V: Into<Json>>(&mut self, key: &str, value: V) -> JsonResult<()> {
        match self {
            Json::Object(map) => {
                map.insert(key.to_string(), value.into());
                Ok(())
            }
            _ => Err(JsonError::new(JsonErrorCode::TypeMismatch)),
        }
    }

    fn remove_field(&mut self, key: &str) -> JsonResult<Option<Json>> {
        match self {
            Json::Object(map) => Ok(map.remove(key)),
            _ => Err(JsonError::new(JsonErrorCode::TypeMismatch)),
        }
    }

    fn push<V: Into<Json>>(&mut self, value: V) -> JsonResult<()> {
        match self {
            Json::Array(arr) => {
                arr.push(value.into());
                Ok(())
            }
            _ => Err(JsonError::new(JsonErrorCode::TypeMismatch)),
        }
    }

    fn insert_at<V: Into<Json>>(&mut self, index: usize, value: V) -> JsonResult<()> {
        match self {
            Json::Array(arr) => {
                if index > arr.len() {
                    return Err(JsonError::new(JsonErrorCode::IndexOutOfBounds));
                }
                arr.insert(index, value.into());
                Ok(())
            }
            _ => Err(JsonError::new(JsonErrorCode::TypeMismatch)),
        }
    }

    fn remove_at(&mut self, index: usize) -> JsonResult<Json> {
        match self {
            Json::Array(arr) => {
                if index >= arr.len() {
                    return Err(JsonError::new(JsonErrorCode::IndexOutOfBounds));
                }
                Ok(arr.remove(index))
            }
            _ => Err(JsonError::new(JsonErrorCode::TypeMismatch)),
        }
    }
}
