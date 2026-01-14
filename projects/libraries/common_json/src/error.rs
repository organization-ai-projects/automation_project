//! Documentation déplacée dans le fichier [error.md](../docs/error.md).

use thiserror::Error;

/// Type d'erreur complet pour les opérations JSON.
///
/// Fournit des variantes spécifiques pour chaque type d'erreur possible,
/// avec des informations contextuelles pour faciliter le débogage.
#[derive(Debug, Error)]
pub enum JsonError {
    /// Erreur de sérialisation ou désérialisation JSON.
    ///
    /// Encapsule les erreurs spécifiques au parsing ou à la sérialisation.
    #[error("Serialization or parsing error: {0}")]
    Serialize(String),

    /// Incompatibilité de type lors de l'accès à une valeur.
    ///
    /// Se produit quand on essaie d'accéder à une valeur JSON avec
    /// un type différent de son type réel.
    ///
    /// # Champs
    /// - `expected`: Le type attendu (ex: "string", "number", "array")
    /// - `found`: Le type réel de la valeur
    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch {
        expected: &'static str,
        found: &'static str,
    },

    /// Champ manquant dans un objet JSON.
    ///
    /// Se produit lors de l'accès à un champ qui n'existe pas dans l'objet.
    #[error("Missing field: {field}")]
    MissingField { field: String },

    /// Index hors limites dans un tableau JSON.
    ///
    /// # Champs
    /// - `index`: L'index demandé
    /// - `length`: La taille réelle du tableau
    #[error("Index out of bounds: {index} (array length: {length})")]
    IndexOutOfBounds { index: usize, length: usize },

    /// Expression de chemin invalide.
    ///
    /// Se produit lors de l'utilisation de `get_path()` avec une syntaxe invalide.
    #[error("Invalid path: {path}")]
    InvalidPath { path: String },

    /// Valeur null rencontrée alors qu'une valeur non-null était attendue.
    #[error("Unexpected null value at {path}")]
    UnexpectedNull { path: String },

    /// Erreur de parsing avec informations de position.
    ///
    /// Permet de localiser précisément l'erreur dans le JSON source.
    #[error("Parse error at line {line}, column {column}: {message}")]
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },

    /// Erreur personnalisée avec message libre.
    ///
    /// Utilisez pour les erreurs métier spécifiques.
    #[error("{0}")]
    Custom(String),

    /// Opération non supportée.
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

impl JsonError {
    /// Crée une erreur de sérialisation ou parsing.
    pub fn serialize<S: Into<String>>(message: S) -> Self {
        Self::Serialize(message.into())
    }

    /// Crée une erreur d'incompatibilité de type.
    ///
    /// # Arguments
    ///
    /// * `expected` - Le type attendu (doit être `&'static str`)
    /// * `found` - Le type trouvé
    pub fn type_mismatch(expected: &'static str, found: &'static str) -> Self {
        Self::TypeMismatch { expected, found }
    }

    /// Crée une erreur de champ manquant.
    pub fn missing_field<S: Into<String>>(field: S) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    /// Crée une erreur d'index hors limites.
    pub fn index_out_of_bounds(index: usize, length: usize) -> Self {
        Self::IndexOutOfBounds { index, length }
    }

    /// Crée une erreur de chemin invalide.
    pub fn invalid_path<S: Into<String>>(path: S) -> Self {
        Self::InvalidPath { path: path.into() }
    }

    /// Crée une erreur de valeur null inattendue.
    pub fn unexpected_null<S: Into<String>>(path: S) -> Self {
        Self::UnexpectedNull { path: path.into() }
    }

    /// Crée une erreur personnalisée.
    pub fn custom<S: Into<String>>(message: S) -> Self {
        Self::Custom(message.into())
    }

    /// Conversion d'une erreur générique en `JsonError`.
    pub fn from_generic_error(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Custom(error.to_string())
    }

    /// Vérifie si c'est une erreur d'incompatibilité de type.
    pub fn is_type_mismatch(&self) -> bool {
        matches!(self, Self::TypeMismatch { .. })
    }

    /// Vérifie si c'est une erreur de champ manquant.
    pub fn is_missing_field(&self) -> bool {
        matches!(self, Self::MissingField { .. })
    }

    /// Vérifie si c'est une erreur de sérialisation.
    pub fn is_serialize(&self) -> bool {
        matches!(self, Self::Serialize(_))
    }
}

/// Type alias pour les résultats d'opérations JSON.
///
/// Équivalent à `Result<T, JsonError>`.
pub type JsonResult<T> = Result<T, JsonError>;
