// projects/libraries/common_json/src/serialization/json_serializable.rs
use crate::{
    Json,
    json_error::{JsonError, JsonResult},
};
use serde::ser::Serialize;
use std::io::Write;

use super::helpers::{json_to_string, serialize_to_json};

/// Trait for types serializable to JSON.
///
/// Automatically implemented for any type `T: Serialize` through the blanket implementation.
/// You do not need to implement this trait manually.
pub trait JsonSerializable {
    /// Converts to a JSON value.
    fn to_json(&self) -> Result<Json, JsonError>;

    /// Converts to a compact JSON string (no spaces).
    fn to_json_string(&self) -> Result<String, JsonError>;

    /// Converts to a formatted JSON string (with indentation).
    fn to_json_string_pretty(&self) -> Result<String, JsonError>;

    /// Converts to JSON bytes (UTF-8).
    fn to_json_bytes(&self) -> Result<Vec<u8>, JsonError>;

    /// Writes JSON to a writer.
    fn write_json<W: Write>(&self, writer: W) -> Result<(), JsonError>;

    /// Writes formatted JSON to a writer.
    fn write_json_pretty<W: Write>(&self, writer: W) -> Result<(), JsonError>;
}

impl<T: Serialize> JsonSerializable for T {
    fn to_json(&self) -> Result<Json, JsonError> {
        serialize_to_json(self)
    }

    fn to_json_string(&self) -> Result<String, JsonError> {
        json_to_string(&serialize_to_json(self)?, false)
    }

    fn to_json_string_pretty(&self) -> Result<String, JsonError> {
        json_to_string(&serialize_to_json(self)?, true)
    }

    fn to_json_bytes(&self) -> Result<Vec<u8>, JsonError> {
        Ok(json_to_string(&serialize_to_json(self)?, false)?.into_bytes())
    }

    fn write_json<W: Write>(&self, mut writer: W) -> Result<(), JsonError> {
        let payload = json_to_string(&serialize_to_json(self)?, false)?;
        writer.write_all(payload.as_bytes())?;
        Ok(())
    }

    fn write_json_pretty<W: Write>(&self, mut writer: W) -> Result<(), JsonError> {
        let payload = json_to_string(&serialize_to_json(self)?, true)?;
        writer.write_all(payload.as_bytes())?;
        Ok(())
    }
}

// ============================================================================
// Standalone functions
// ============================================================================

/// Converts a value to `Json`.
#[inline]
pub fn to_json<T: Serialize>(value: &T) -> Result<Json, JsonError> {
    serialize_to_json(value)
}

/// Converts a value to a compact JSON string.
#[inline]
pub fn to_string<T: Serialize>(value: &T) -> JsonResult<String> {
    json_to_string(&serialize_to_json(value)?, false)
}

/// Converts a value to a formatted JSON string (with indentation).
#[inline]
pub fn to_string_pretty<T: Serialize>(value: &T) -> JsonResult<String> {
    json_to_string(&serialize_to_json(value)?, true)
}

/// Converts a value to JSON bytes (UTF-8).
#[inline]
pub fn to_bytes<T: Serialize>(value: &T) -> JsonResult<Vec<u8>> {
    Ok(json_to_string(&serialize_to_json(value)?, false)?.into_bytes())
}

/// Converts a value to formatted JSON bytes.
#[inline]
pub fn to_bytes_pretty<T: Serialize>(value: &T) -> JsonResult<Vec<u8>> {
    Ok(json_to_string(&serialize_to_json(value)?, true)?.into_bytes())
}

/// Writes JSON to a writer.
#[inline]
pub fn write_to<T: Serialize, W: Write>(value: &T, mut writer: W) -> JsonResult<()> {
    let payload = json_to_string(&serialize_to_json(value)?, false)?;
    writer.write_all(payload.as_bytes())?;
    Ok(())
}

/// Writes formatted JSON to a writer.
#[inline]
pub fn write_to_pretty<T: Serialize, W: Write>(value: &T, mut writer: W) -> JsonResult<()> {
    let payload = json_to_string(&serialize_to_json(value)?, true)?;
    writer.write_all(payload.as_bytes())?;
    Ok(())
}

/// Converts a `Json` value to formatted JSON bytes.
#[inline]
pub fn value_to_bytes_pretty(json: &Json) -> Result<Vec<u8>, JsonError> {
    Ok(json_to_string(json, true)?.into_bytes())
}

// ============================================================================
// Legacy aliases (backward compatibility)
// ============================================================================

/// Legacy alias for [`to_json`].
#[inline]
pub fn to_value<T: Serialize>(value: &T) -> Result<Json, JsonError> {
    to_json(value)
}

/// Legacy alias - converts to a JSON string.
#[inline]
pub fn to_json_string<T: Serialize>(value: &T) -> JsonResult<String> {
    to_string(value)
}

/// Legacy alias - converts to a formatted JSON string.
#[inline]
pub fn to_json_string_pretty<T: Serialize>(value: &T) -> JsonResult<String> {
    to_string_pretty(value)
}
