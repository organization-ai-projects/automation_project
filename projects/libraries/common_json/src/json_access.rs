// projects/libraries/common_json/src/json_access.rs
use crate::Json;
use crate::{JsonArray, JsonObject, JsonResult};

pub trait JsonAccess {
    fn get_field(&self, key: &str) -> JsonResult<&Json>;
    fn get_index(&self, index: usize) -> JsonResult<&Json>;
    fn get_path(&self, path: &str) -> JsonResult<&Json>;
    fn as_str_strict(&self) -> JsonResult<&str>;
    fn as_i64_strict(&self) -> JsonResult<i64>;
    fn as_u64_strict(&self) -> JsonResult<u64>;
    fn as_f64_strict(&self) -> JsonResult<f64>;
    fn as_bool_strict(&self) -> JsonResult<bool>;
    fn as_array_strict(&self) -> JsonResult<&JsonArray>;
    fn as_object_strict(&self) -> JsonResult<&JsonObject>;
    fn type_name(&self) -> &'static str;
    fn is_truthy(&self) -> bool;
}
