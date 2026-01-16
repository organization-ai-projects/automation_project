use crate::{
    Json,
    error::{JsonError, JsonResult},
};
use serde::ser::Serialize;
use std::io::Write;

use super::helpers::{json_to_string, serialize_to_json};

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
pub fn to_json<T: Serialize>(value: &T) -> Result<Json, JsonError> {
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
