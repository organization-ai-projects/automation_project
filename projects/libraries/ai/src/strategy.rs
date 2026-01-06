/// Stratégies d'orchestration neuro-symbolique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolverStrategy {
    /// Utiliser uniquement le solver symbolique
    SymbolicOnly,

    /// Utiliser uniquement le solver neural
    NeuralOnly,

    /// Essayer symbolic d'abord, si échec ou confiance basse → neural
    SymbolicThenNeural,

    /// Neural génère, symbolic valide et corrige si nécessaire
    NeuralWithSymbolicValidation,

    /// Exécuter les deux en parallèle, choisir le meilleur résultat
    Hybrid,
}
