use crate::Json;

/// Représente une différence dans un élément de tableau.
#[derive(Debug, Clone, PartialEq)]
pub enum ArrayDiff {
    /// Élément identique.
    Same(Json),
    /// Élément ajouté.
    Added(Json),
    /// Élément supprimé.
    Removed(Json),
    /// Élément modifié.
    Changed {
        /// Ancienne valeur.
        from: Json,
        /// Nouvelle valeur.
        to: Json,
    },
}
