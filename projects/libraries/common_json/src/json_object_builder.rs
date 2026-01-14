use crate::{Json, JsonMap};

/// Builder fluide pour construire des objets JSON.
///
/// Permet de construire des objets JSON de manière lisible et type-safe.
pub struct JsonObjectBuilder {
    map: JsonMap,
}

impl JsonObjectBuilder {
    /// Crée un nouveau builder vide.
    pub fn new() -> Self {
        Self {
            map: JsonMap::new(),
        }
    }

    /// Ajoute un champ à l'objet.
    ///
    /// La clé et la valeur sont convertis via `Into<String>` et `Into<Json>`.
    pub fn field<K: Into<String>, V: Into<Json>>(mut self, key: K, value: V) -> Self {
        self.map.insert(key.into(), value.into());
        self
    }

    /// Ajoute un champ seulement si la valeur est `Some`.
    ///
    /// Si `None`, le champ n'est pas ajouté.
    pub fn field_opt<K: Into<String>, V: Into<Json>>(mut self, key: K, value: Option<V>) -> Self {
        if let Some(v) = value {
            self.map.insert(key.into(), v.into());
        }
        self
    }

    /// Ajoute un champ seulement si la condition est vraie.
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

    /// Finalise et retourne l'objet JSON.
    pub fn build(self) -> Json {
        Json::Object(self.map)
    }
}

impl Default for JsonObjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}
