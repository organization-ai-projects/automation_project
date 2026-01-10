/// Type de feedback que l'utilisateur peut donner
#[derive(Debug, Clone)]
pub enum FeedbackType {
    /// Code généré était correct
    Correct,
    /// Code généré était incorrect, voici la bonne version
    Incorrect { expected_output: String },
    /// Code était partiellement correct, ajustement nécessaire
    Partial { correction: String, confidence: f32 },
}
