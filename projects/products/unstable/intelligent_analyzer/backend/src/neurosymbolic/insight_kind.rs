use serde::{Deserialize, Serialize};

/// Kind of AI-generated insight.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InsightKind {
    Suggestion,
    Refactoring,
    PatternDetection,
    ComplexityWarning,
}

impl std::fmt::Display for InsightKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Suggestion => write!(f, "suggestion"),
            Self::Refactoring => write!(f, "refactoring"),
            Self::PatternDetection => write!(f, "pattern_detection"),
            Self::ComplexityWarning => write!(f, "complexity_warning"),
        }
    }
}
