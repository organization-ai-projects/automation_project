use std::collections::HashMap;

use crate::array_diff::ArrayDiff;
use crate::json::Json;

/// Représente une différence entre deux valeurs JSON.
///
/// Résultat de la fonction [`diff`].
#[derive(Debug, Clone, PartialEq)]
pub enum JsonDiff {
    /// Pas de différence.
    Same,
    /// Valeur ajoutée (présente uniquement dans source).
    Added(Json),
    /// Valeur supprimée (présente uniquement dans target).
    Removed(Json),
    /// Valeur modifiée.
    Changed {
        /// Ancienne valeur (dans target).
        from: Json,
        /// Nouvelle valeur (dans source).
        to: Json,
    },
    /// Objet avec des différences par champ.
    Object(HashMap<String, JsonDiff>),
    /// Tableau avec des différences par élément.
    Array(Vec<ArrayDiff>),
}
