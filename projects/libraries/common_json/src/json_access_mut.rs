// projects/libraries/common_json/src/json_access_mut.rs
use crate::{JsonResult, json::Json};

pub trait JsonAccessMut {
    /// Returns a mutable reference to a field.
    fn get_field_mut(&mut self, key: &str) -> JsonResult<&mut Json>;

    /// Returns a mutable reference to an element by index.
    fn get_index_mut(&mut self, index: usize) -> JsonResult<&mut Json>;

    /// Sets a field in an object (creates or replaces).
    fn set_field<V: Into<Json>>(&mut self, key: &str, value: V) -> JsonResult<()>;

    /// Removes a field from an object.
    fn remove_field(&mut self, key: &str) -> JsonResult<Option<Json>>;

    /// Appends a value to the end of an array.
    fn push<V: Into<Json>>(&mut self, value: V) -> JsonResult<()>;

    /// Inserts a value at a specific index in an array.
    fn insert_at<V: Into<Json>>(&mut self, index: usize, value: V) -> JsonResult<()>;

    /// Removes and returns the element at the given index.
    fn remove_at(&mut self, index: usize) -> JsonResult<Json>;
}
