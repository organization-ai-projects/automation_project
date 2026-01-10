/// Structure représentant un retour utilisateur pour le solveur symbolique
pub struct SymbolicFeedback {
    pub is_positive: bool,
    pub metadata: Option<String>,
}

impl SymbolicFeedback {
    pub fn new(is_positive: bool, metadata: Option<String>) -> Self {
        Self {
            is_positive,
            metadata,
        }
    }

    pub fn is_positive(&self) -> bool {
        self.is_positive
    }
}
