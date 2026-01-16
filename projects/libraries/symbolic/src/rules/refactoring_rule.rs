#[derive(Debug, Clone)]
pub struct RefactoringRule {
    pub name: String,
    pub pattern: String,
    pub replacement: String,
    pub description: String,
}
