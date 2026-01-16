use crate::Json;

/// Opération de patch JSON (style RFC 6902).
///
/// **Note** : Ce type est défini mais les opérations ne sont pas encore implémentées.
#[derive(Debug, Clone)]
pub enum PatchOp {
    /// Ajoute une valeur à un chemin.
    Add {
        /// Chemin JSON Pointer.
        path: String,
        /// Valeur à ajouter.
        value: Json,
    },
    /// Supprime la valeur à un chemin.
    Remove {
        /// Chemin JSON Pointer.
        path: String,
    },
    /// Remplace la valeur à un chemin.
    Replace {
        /// Chemin JSON Pointer.
        path: String,
        /// Nouvelle valeur.
        value: Json,
    },
    /// Déplace une valeur d'un chemin à un autre.
    Move {
        /// Chemin source.
        from: String,
        /// Chemin destination.
        to: String,
    },
    /// Copie une valeur d'un chemin à un autre.
    Copy {
        /// Chemin source.
        from: String,
        /// Chemin destination.
        to: String,
    },
}
