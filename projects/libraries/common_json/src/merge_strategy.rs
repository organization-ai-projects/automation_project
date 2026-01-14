/// Stratégie de fusion pour combiner des valeurs JSON.
///
/// Détermine comment les valeurs sont combinées lors d'un merge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MergeStrategy {
    /// Remplace la cible par la source (pas de fusion).
    #[default]
    Replace,
    /// Fusionne récursivement les objets, remplace les autres types.
    DeepMerge,
    /// Concatène les tableaux, fusionne récursivement les objets.
    Concat,
}
