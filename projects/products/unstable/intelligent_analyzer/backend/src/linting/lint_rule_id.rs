use serde::{Deserialize, Serialize};

/// Identifies a lint rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LintRuleId {
    TrailingWhitespace,
    LineTooLong,
    TodoComment,
    UnusedImport,
    MissingDocComment,
    Custom(String),
}

impl std::fmt::Display for LintRuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TrailingWhitespace => write!(f, "trailing_whitespace"),
            Self::LineTooLong => write!(f, "line_too_long"),
            Self::TodoComment => write!(f, "todo_comment"),
            Self::UnusedImport => write!(f, "unused_import"),
            Self::MissingDocComment => write!(f, "missing_doc_comment"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}
