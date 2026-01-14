use crate::{Json, JsonResult};

pub trait JsonAccessMut {
    /// Retourne une référence mutable vers un champ.
    fn get_field_mut(&mut self, key: &str) -> JsonResult<&mut Json>;

    /// Retourne une référence mutable vers un élément par index.
    fn get_index_mut(&mut self, index: usize) -> JsonResult<&mut Json>;

    /// Définit un champ dans un objet (crée ou remplace).
    ///
    /// Accepte tout type implémentant `Into<Json>`.
    fn set_field<V: Into<Json>>(&mut self, key: &str, value: V) -> JsonResult<()>;

    /// Supprime un champ d'un objet.
    ///
    /// Retourne `Some(valeur)` si le champ existait, `None` sinon.
    fn remove_field(&mut self, key: &str) -> JsonResult<Option<Json>>;

    /// Ajoute une valeur à la fin d'un tableau.
    fn push<V: Into<Json>>(&mut self, value: V) -> JsonResult<()>;

    /// Insère une valeur à un index dans un tableau.
    ///
    /// Les éléments suivants sont décalés vers la droite.
    fn insert_at<V: Into<Json>>(&mut self, index: usize, value: V) -> JsonResult<()>;

    /// Supprime et retourne l'élément à l'index donné.
    fn remove_at(&mut self, index: usize) -> JsonResult<Json>;
}
