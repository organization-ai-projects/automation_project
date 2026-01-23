/// projects/libraries/symbolic/src/rules/code_template.rs
/// Code generation template
#[derive(Debug, Clone)]
pub struct CodeTemplate {
    pub pattern: String,
    pub template: String,
    pub confidence: f64,
}
