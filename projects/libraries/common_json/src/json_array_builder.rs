use crate::Json;

/// Builder fluide pour construire des tableaux JSON.
///
/// Permet de construire des tableaux JSON de manière lisible et type-safe.
pub struct JsonArrayBuilder {
    arr: Vec<Json>,
}

impl JsonArrayBuilder {
    /// Crée un nouveau builder vide.
    pub fn new() -> Self {
        Self { arr: Vec::new() }
    }

    /// Crée un builder avec une capacité pré-allouée.
    ///
    /// Utile si vous connaissez le nombre approximatif d'éléments.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arr: Vec::with_capacity(capacity),
        }
    }

    /// Ajoute un élément au tableau.
    pub fn element<V: Into<Json>>(mut self, value: V) -> Self {
        self.arr.push(value.into());
        self
    }

    /// Ajoute un élément seulement si la valeur est `Some`.
    pub fn element_opt<V: Into<Json>>(mut self, value: Option<V>) -> Self {
        if let Some(v) = value {
            self.arr.push(v.into());
        }
        self
    }

    /// Ajoute un élément seulement si la condition est vraie.
    pub fn element_if<V: Into<Json>>(self, condition: bool, value: V) -> Self {
        if condition { self.element(value) } else { self }
    }

    /// Étend le tableau avec plusieurs éléments.
    ///
    /// Accepte tout itérateur d'éléments convertibles en `Json`.
    pub fn extend<I, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<Json>,
    {
        self.arr.extend(iter.into_iter().map(Into::into));
        self
    }

    /// Finalise et retourne le tableau JSON.
    pub fn build(self) -> Json {
        Json::Array(self.arr)
    }
}

impl Default for JsonArrayBuilder {
    fn default() -> Self {
        Self::new()
    }
}
