/// Résultat d'un refactoring
#[derive(Debug, Clone)]
pub struct RefactoringResult {
    pub code: String,
    pub confidence: f64,
    pub changes_applied: Vec<String>,
}