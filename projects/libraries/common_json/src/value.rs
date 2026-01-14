// projects/libraries/common_json/src/value.rs
// ============================================================================
// Type Aliases
// ============================================================================

use serde::{Deserialize, Serialize};

use crate::JsonError;

/// Valeur JSON générique.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Json {
    Null,
    Bool(bool),
    Number(JsonNumber),
    String(String),
    Array(JsonArray),
    Object(JsonMap),
}

/// Map clé-valeur pour les objets JSON.
pub type JsonMap = std::collections::HashMap<String, Json>;

/// Tableau de valeurs JSON.
pub type JsonArray = Vec<Json>;

/// Objet JSON (map String -> Json).
pub type JsonObject = JsonMap;

/// Nombre JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonNumber {
    value: f64,
}

impl JsonNumber {
    pub fn from_f64(n: f64) -> Option<Self> {
        if n.is_finite() {
            Some(JsonNumber { value: n })
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> f64 {
        self.value
    }

    pub fn is_non_zero(&self) -> bool {
        self.value != 0.0
    }
}

impl From<i64> for JsonNumber {
    fn from(value: i64) -> Self {
        JsonNumber {
            value: value as f64,
        }
    }
}

impl From<u64> for JsonNumber {
    fn from(value: u64) -> Self {
        JsonNumber {
            value: value as f64,
        }
    }
}

// ============================================================================
// Constructors
// ============================================================================

/// Crée un objet JSON vide `{}`.
#[inline]
pub fn object() -> Json {
    Json::Object(JsonMap::new())
}

/// Crée un tableau JSON vide `[]`.
#[inline]
pub fn array() -> Json {
    Json::Array(Vec::new())
}

/// Crée une valeur JSON `null`.
#[inline]
pub fn null() -> Json {
    Json::Null
}

/// Crée un booléen JSON.
#[inline]
pub fn boolean(v: bool) -> Json {
    Json::Bool(v)
}

/// Crée une chaîne JSON.
#[inline]
pub fn string<S: Into<String>>(s: S) -> Json {
    Json::String(s.into())
}

/// Crée un nombre JSON à partir d'un `i64`.
#[inline]
pub fn number_i64(n: i64) -> Json {
    Json::Number(n.into())
}

/// Crée un nombre JSON à partir d'un `u64`.
#[inline]
pub fn number_u64(n: u64) -> Json {
    Json::Number(n.into())
}

/// Crée un nombre JSON à partir d'un `f64`.
#[inline]
pub fn number_f64(n: f64) -> Option<Json> {
    JsonNumber::from_f64(n).map(Json::Number)
}

impl Json {
    pub fn as_str(&self) -> Option<&str> {
        if let Json::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        if let Json::Number(n) = self {
            Some(n.value as i64)
        } else {
            None
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        if let Json::Number(n) = self {
            Some(n.value as u64)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let Json::Number(n) = self {
            Some(n.value)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Json::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&JsonArray> {
        if let Json::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<&JsonMap> {
        if let Json::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

    pub fn is_non_zero(&self) -> bool {
        if let Json::Number(n) = self {
            n.is_non_zero()
        } else {
            false
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Json::Object(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Json::Array(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }

    /// Convertit un type sérialisable en `Json`.
    pub fn from_serialize<T: Serialize>(value: &T) -> Result<Self, JsonError> {
        crate::serialize::to_json(value)
    }
}

impl From<&str> for Json {
    fn from(value: &str) -> Self {
        Json::String(value.to_string())
    }
}

impl From<String> for Json {
    fn from(value: String) -> Self {
        Json::String(value)
    }
}

impl From<i64> for Json {
    fn from(value: i64) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<u64> for Json {
    fn from(value: u64) -> Self {
        Json::Number(JsonNumber {
            value: value as f64,
        })
    }
}

impl From<f64> for Json {
    fn from(value: f64) -> Self {
        Json::Number(JsonNumber { value })
    }
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        Json::Bool(value)
    }
}
