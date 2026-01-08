/// Template de génération de code
#[derive(Debug, Clone)]
pub struct CodeTemplate {
    pub pattern: String,
    pub template: String,
    pub confidence: f64,
}
