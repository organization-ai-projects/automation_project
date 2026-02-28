#[derive(Debug, Clone)]
pub struct ForbiddenRule {
    pub pattern: String,
}

impl ForbiddenRule {
    pub fn new(pattern: impl Into<String>) -> Self {
        Self {
            pattern: pattern.into(),
        }
    }

    pub fn matches(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }
}
