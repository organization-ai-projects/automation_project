use serde::{Deserialize, Serialize};

/// Kind of finding produced by the analysis engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FindingKind {
    UnusedVariable,
    UnusedSymbol,
    UndefinedSymbol,
    TypeMismatch,
    UnreachableCode,
    MissingSemicolon,
    ScopeViolation,
    Custom(String),
}

impl std::fmt::Display for FindingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnusedVariable => write!(f, "unused_variable"),
            Self::UnusedSymbol => write!(f, "unused_symbol"),
            Self::UndefinedSymbol => write!(f, "undefined_symbol"),
            Self::TypeMismatch => write!(f, "type_mismatch"),
            Self::UnreachableCode => write!(f, "unreachable_code"),
            Self::MissingSemicolon => write!(f, "missing_semicolon"),
            Self::ScopeViolation => write!(f, "scope_violation"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}
