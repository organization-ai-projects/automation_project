// projects/libraries/common_json/src/json_array_builder.rs
use crate::json::Json;

/// Fluent builder for constructing JSON arrays.
///
/// Allows building JSON arrays in a readable and type-safe manner.
pub struct JsonArrayBuilder {
    arr: Vec<Json>,
}

impl JsonArrayBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self { arr: Vec::new() }
    }

    /// Creates a builder with a pre-allocated capacity.
    ///
    /// Useful if you know the approximate number of elements.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arr: Vec::with_capacity(capacity),
        }
    }

    /// Adds an element to the array.
    pub fn element<V: Into<Json>>(mut self, value: V) -> Self {
        self.arr.push(value.into());
        self
    }

    /// Adds an element only if the value is `Some`.
    pub fn element_opt<V: Into<Json>>(mut self, value: Option<V>) -> Self {
        if let Some(v) = value {
            self.arr.push(v.into());
        }
        self
    }

    /// Adds an element only if the condition is true.
    pub fn element_if<V: Into<Json>>(self, condition: bool, value: V) -> Self {
        if condition { self.element(value) } else { self }
    }

    /// Extends the array with multiple elements.
    ///
    /// Accepts any iterator of elements convertible to `Json`.
    pub fn extend<I, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<Json>,
    {
        self.arr.extend(iter.into_iter().map(Into::into));
        self
    }

    /// Finalizes and returns the JSON array.
    pub fn build(self) -> Json {
        Json::Array(self.arr)
    }
}

impl Default for JsonArrayBuilder {
    fn default() -> Self {
        Self::new()
    }
}
