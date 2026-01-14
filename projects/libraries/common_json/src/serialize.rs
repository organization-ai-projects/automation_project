use crate::error::{JsonError, JsonResult};
use crate::value::{Json, JsonMap, JsonNumber};
use serde::ser::{
    self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
use std::fmt;
use std::io::Write;

const INDENT_WIDTH: usize = 2;

impl ser::Error for JsonError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        JsonError::custom(msg.to_string())
    }
}

struct JsonSerializer;

impl ser::Serializer for JsonSerializer {
    type Ok = Json;
    type Error = JsonError;

    type SerializeSeq = JsonSeqSerializer;
    type SerializeTuple = JsonSeqSerializer;
    type SerializeTupleStruct = JsonSeqSerializer;
    type SerializeTupleVariant = JsonTupleVariantSerializer;
    type SerializeMap = JsonMapSerializer;
    type SerializeStruct = JsonMapSerializer;
    type SerializeStructVariant = JsonStructVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Number(JsonNumber::from(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Number(JsonNumber::from(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if !v.is_finite() {
            return Err(JsonError::serialize("non-finite number"));
        }
        let number =
            JsonNumber::from_f64(v).ok_or_else(|| JsonError::serialize("non-finite number"))?;
        Ok(Json::Number(number))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Json::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Json::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let values = v
            .iter()
            .map(|b| Json::Number(JsonNumber::from(*b as u64)))
            .collect();
        Ok(Json::Array(values))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Null)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Json::Null)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Json::String(variant.to_string()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        let mut map = JsonMap::new();
        map.insert(variant.to_string(), value.serialize(self)?);
        Ok(Json::Object(map))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(JsonSeqSerializer {
            elements: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(JsonTupleVariantSerializer {
            name: variant.to_string(),
            elements: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(JsonMapSerializer::with_capacity(len.unwrap_or(0)))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(JsonStructVariantSerializer {
            name: variant.to_string(),
            map: JsonMap::with_capacity(len),
        })
    }
}

struct JsonSeqSerializer {
    elements: Vec<Json>,
}

impl SerializeSeq for JsonSeqSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        self.elements.push(value.serialize(JsonSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Json, JsonError> {
        Ok(Json::Array(self.elements))
    }
}

impl SerializeTuple for JsonSeqSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Json, JsonError> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for JsonSeqSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Json, JsonError> {
        SerializeSeq::end(self)
    }
}

struct JsonTupleVariantSerializer {
    name: String,
    elements: Vec<Json>,
}

impl SerializeTupleVariant for JsonTupleVariantSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        self.elements.push(value.serialize(JsonSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Json, JsonError> {
        let mut map = JsonMap::new();
        map.insert(self.name, Json::Array(self.elements));
        Ok(Json::Object(map))
    }
}

struct JsonMapSerializer {
    map: JsonMap,
    next_key: Option<String>,
}

impl JsonMapSerializer {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            map: JsonMap::with_capacity(capacity),
            next_key: None,
        }
    }
}

impl SerializeMap for JsonMapSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), JsonError> {
        let key = key.serialize(KeySerializer)?;
        self.next_key = Some(key);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), JsonError> {
        let key = self
            .next_key
            .take()
            .ok_or_else(|| JsonError::custom("value serialized before key"))?;
        let value = value.serialize(JsonSerializer)?;
        self.map.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Json, JsonError> {
        Ok(Json::Object(self.map))
    }
}

impl SerializeStruct for JsonMapSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), JsonError> {
        self.map
            .insert(key.to_string(), value.serialize(JsonSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Json, JsonError> {
        Ok(Json::Object(self.map))
    }
}

struct JsonStructVariantSerializer {
    name: String,
    map: JsonMap,
}

impl SerializeStructVariant for JsonStructVariantSerializer {
    type Ok = Json;
    type Error = JsonError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), JsonError> {
        self.map
            .insert(key.to_string(), value.serialize(JsonSerializer)?);
        Ok(())
    }

    fn end(self) -> Result<Json, JsonError> {
        let mut wrapper = JsonMap::new();
        wrapper.insert(self.name, Json::Object(self.map));
        Ok(Json::Object(wrapper))
    }
}

struct KeySerializer;

impl ser::Serializer for KeySerializer {
    type Ok = String;
    type Error = JsonError;

    type SerializeSeq = ser::Impossible<String, JsonError>;
    type SerializeTuple = ser::Impossible<String, JsonError>;
    type SerializeTupleStruct = ser::Impossible<String, JsonError>;
    type SerializeTupleVariant = ser::Impossible<String, JsonError>;
    type SerializeMap = ser::Impossible<String, JsonError>;
    type SerializeStruct = ser::Impossible<String, JsonError>;
    type SerializeStructVariant = ser::Impossible<String, JsonError>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if !v.is_finite() {
            return Err(JsonError::serialize("non-finite number"));
        }
        Ok(v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(JsonError::custom("JSON object keys cannot be bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(JsonError::custom("JSON object keys cannot be null"))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(JsonError::custom(
            "JSON object keys must be primitive types",
        ))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(JsonError::custom("JSON object keys cannot be unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(JsonError::custom("JSON object keys cannot be unit"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(variant.to_string())
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(JsonError::custom(
            "JSON object keys must be primitive types",
        ))
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(variant.to_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(JsonError::custom(
            "JSON object keys must be primitive types",
        ))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(JsonError::custom(
            "JSON object keys must be primitive types",
        ))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(JsonError::custom(
            "JSON object keys must be primitive types",
        ))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(JsonError::custom(
            "JSON object keys must be primitive types",
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(JsonError::custom(
            "JSON object keys must be primitive types",
        ))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(JsonError::custom(
            "JSON object keys must be primitive types",
        ))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(JsonError::custom(
            "JSON object keys must be primitive types",
        ))
    }
}

fn serialize_to_json<T: Serialize>(value: &T) -> Result<Json, JsonError> {
    value.serialize(JsonSerializer)
}

fn json_to_string(json: &Json, pretty: bool) -> JsonResult<String> {
    let mut output = String::new();
    append_json(&mut output, json, pretty, 0)?;
    Ok(output)
}

fn append_json(output: &mut String, json: &Json, pretty: bool, indent: usize) -> JsonResult<()> {
    match json {
        Json::Null => output.push_str("null"),
        Json::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
        Json::Number(number) => {
            let value = number.as_f64();
            if !value.is_finite() {
                return Err(JsonError::serialize("non-finite number"));
            }
            output.push_str(&value.to_string());
        }
        Json::String(value) => push_escaped_string(output, value),
        Json::Array(values) => {
            if values.is_empty() {
                output.push_str("[]");
            } else {
                output.push('[');
                if pretty {
                    output.push('\n');
                }
                for (idx, value) in values.iter().enumerate() {
                    if pretty {
                        push_indent(output, indent + INDENT_WIDTH);
                    }
                    append_json(output, value, pretty, indent + INDENT_WIDTH)?;
                    if idx + 1 < values.len() {
                        output.push(',');
                    }
                    if pretty {
                        output.push('\n');
                    }
                }
                if pretty {
                    push_indent(output, indent);
                }
                output.push(']');
            }
        }
        Json::Object(map) => {
            if map.is_empty() {
                output.push_str("{}");
            } else {
                output.push('{');
                if pretty {
                    output.push('\n');
                }
                for (idx, (key, value)) in map.iter().enumerate() {
                    if pretty {
                        push_indent(output, indent + INDENT_WIDTH);
                    }
                    push_escaped_string(output, key);
                    output.push(':');
                    if pretty {
                        output.push(' ');
                    }
                    append_json(output, value, pretty, indent + INDENT_WIDTH)?;
                    if idx + 1 < map.len() {
                        output.push(',');
                    }
                    if pretty {
                        output.push('\n');
                    }
                }
                if pretty {
                    push_indent(output, indent);
                }
                output.push('}');
            }
        }
    }
    Ok(())
}

fn push_indent(output: &mut String, indent: usize) {
    output.push_str(&" ".repeat(indent));
}

fn push_escaped_string(output: &mut String, value: &str) {
    output.push('"');
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0C}' => output.push_str("\\f"),
            ch if ch <= '\u{1F}' => {
                let _ = write!(output, "\\u{:04x}", ch as u32);
            }
            _ => output.push(ch),
        }
    }
    output.push('"');
}

/// Trait pour les types sérialisables en JSON.
///
/// Implémentété automatiquement pour tout type `T: Serialize` grâce à
/// l'implémentation blanket. Vous n'avez pas besoin d'implémenter ce trait
/// manuellement.
pub trait JsonSerializable {
    /// Convertit en valeur JSON.
    fn to_json(&self) -> Result<Json, JsonError>;

    /// Convertit en chaîne JSON compacte (sans espaces).
    fn to_json_string(&self) -> Result<String, JsonError>;

    /// Convertit en chaîne JSON formatée (avec indentation).
    fn to_json_string_pretty(&self) -> Result<String, JsonError>;

    /// Convertit en bytes JSON (UTF-8).
    fn to_json_bytes(&self) -> Result<Vec<u8>, JsonError>;

    /// Écrit le JSON dans un writer.
    fn write_json<W: Write>(&self, writer: W) -> Result<(), JsonError>;

    /// Écrit le JSON formaté dans un writer.
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
// Fonctions standalone
// ============================================================================

/// Convertit une valeur en `Json`.
#[inline]
pub fn to_json<T: Serialize>(value: &T) -> Result<Json, crate::error::JsonError> {
    serialize_to_json(value)
}

/// Convertit une valeur en chaîne JSON compacte.
#[inline]
pub fn to_string<T: Serialize>(value: &T) -> JsonResult<String> {
    json_to_string(&serialize_to_json(value)?, false)
}

/// Convertit une valeur en chaîne JSON formatée (avec indentation).
#[inline]
pub fn to_string_pretty<T: Serialize>(value: &T) -> JsonResult<String> {
    json_to_string(&serialize_to_json(value)?, true)
}

/// Convertit une valeur en bytes JSON (UTF-8).
#[inline]
pub fn to_bytes<T: Serialize>(value: &T) -> JsonResult<Vec<u8>> {
    Ok(json_to_string(&serialize_to_json(value)?, false)?.into_bytes())
}

/// Convertit une valeur en bytes JSON formatés.
#[inline]
pub fn to_bytes_pretty<T: Serialize>(value: &T) -> JsonResult<Vec<u8>> {
    Ok(json_to_string(&serialize_to_json(value)?, true)?.into_bytes())
}

/// Écrit le JSON dans un writer.
#[inline]
pub fn write_to<T: Serialize, W: Write>(value: &T, mut writer: W) -> JsonResult<()> {
    let payload = json_to_string(&serialize_to_json(value)?, false)?;
    writer.write_all(payload.as_bytes())?;
    Ok(())
}

/// Écrit le JSON formaté dans un writer.
#[inline]
pub fn write_to_pretty<T: Serialize, W: Write>(value: &T, mut writer: W) -> JsonResult<()> {
    let payload = json_to_string(&serialize_to_json(value)?, true)?;
    writer.write_all(payload.as_bytes())?;
    Ok(())
}

/// Convertit un `Json` en bytes JSON formatés.
#[inline]
pub fn value_to_bytes_pretty(json: &Json) -> Result<Vec<u8>, JsonError> {
    Ok(json_to_string(json, true)?.into_bytes())
}

// ============================================================================
// Alias legacy (compatibilité ascendante)
// ============================================================================

/// Alias legacy pour [`to_json`].
#[inline]
pub fn to_value<T: Serialize>(value: &T) -> Result<Json, JsonError> {
    to_json(value)
}

/// Alias legacy - convertit en chaîne JSON.
#[inline]
pub fn to_json_string<T: Serialize>(value: &T) -> JsonResult<String> {
    to_string(value)
}

/// Alias legacy - convertit en chaîne JSON formatée.
#[inline]
pub fn to_json_string_pretty<T: Serialize>(value: &T) -> JsonResult<String> {
    to_string_pretty(value)
}
