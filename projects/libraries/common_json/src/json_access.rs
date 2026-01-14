use crate::{Json, JsonArray, JsonObject, JsonResult};

/// Trait d'extension pour l'accès fluide aux valeurs JSON.
///
/// Ce trait est implémenté pour [`Json`] et fournit des méthodes pour :
/// - Accéder aux champs d'objets et éléments de tableaux
/// - Naviguer dans des structures imbriquées par chemin
/// - Convertir avec vérification de type stricte
/// - Inspecter le type et la "véracité" d'une valeur
pub trait JsonAccess {
    /// Accède à un champ d'un objet JSON.
    ///
    /// # Erreurs
    ///
    /// - `JsonError::TypeMismatch` si ce n'est pas un objet
    /// - `JsonError::MissingField` si le champ n'existe pas
    fn get_field(&self, key: &str) -> JsonResult<&Json>;

    /// Accède à un élément d'un tableau JSON par son index.
    ///
    /// # Erreurs
    ///
    /// - `JsonError::TypeMismatch` si ce n'est pas un tableau
    /// - `JsonError::IndexOutOfBounds` si l'index est hors limites
    fn get_index(&self, index: usize) -> JsonResult<&Json>;

    /// Navigue vers une valeur imbriquée avec la notation par points.
    ///
    /// Supporte :
    /// - `"field"` - accès simple
    /// - `"a.b.c"` - navigation imbriquée
    /// - `"arr[0]"` - accès par index
    /// - `"a.b[2].c"` - combinaison
    fn get_path(&self, path: &str) -> JsonResult<&Json>;

    /// Retourne la chaîne si c'est un string, sinon erreur.
    fn as_str_strict(&self) -> JsonResult<&str>;

    /// Retourne l'entier i64 si c'est un nombre, sinon erreur.
    fn as_i64_strict(&self) -> JsonResult<i64>;

    /// Retourne l'entier u64 si c'est un nombre, sinon erreur.
    fn as_u64_strict(&self) -> JsonResult<u64>;

    /// Retourne le flottant f64 si c'est un nombre, sinon erreur.
    fn as_f64_strict(&self) -> JsonResult<f64>;

    /// Retourne le booléen si c'est un bool, sinon erreur.
    fn as_bool_strict(&self) -> JsonResult<bool>;

    /// Retourne le tableau si c'est un array, sinon erreur.
    fn as_array_strict(&self) -> JsonResult<&JsonArray>;

    /// Retourne l'objet si c'est un object, sinon erreur.
    fn as_object_strict(&self) -> JsonResult<&JsonObject>;

    /// Retourne le nom du type JSON.
    ///
    /// Valeurs possibles : `"null"`, `"bool"`, `"number"`, `"string"`, `"array"`, `"object"`.
    fn type_name(&self) -> &'static str;

    /// Vérifie si la valeur est "truthy" (vraie au sens JavaScript).
    ///
    /// | Type | Truthy si... |
    /// |------|--------------|
    /// | null | Jamais |
    /// | bool | `true` |
    /// | number | `!= 0` |
    /// | string | Non vide |
    /// | array | Non vide |
    /// | object | Non vide |
    fn is_truthy(&self) -> bool;
}
