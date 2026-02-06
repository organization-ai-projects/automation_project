// projects/libraries/symbolic/src/validator/semantic_issue.rs

/// Severity level of a semantic issue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Critical issue that should always be an error
    Error,
    /// Warning that can be elevated to error in strict mode
    Warning,
    /// Informational issue
    Info,
}

/// Represents a semantic issue found during code validation
#[derive(Debug, Clone)]
pub struct SemanticIssue {
    /// The type of semantic issue
    pub issue_type: SemanticIssueType,
    /// Severity of the issue
    pub severity: Severity,
    /// Human-readable message describing the issue
    pub message: String,
    /// Optional line number where the issue occurs
    pub line: Option<usize>,
}

/// Types of semantic issues that can be detected
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticIssueType {
    /// An unused variable was found
    UnusedVariable,
    /// An unused import was found
    UnusedImport,
    /// Dead code that will never execute
    DeadCode,
    /// Type inconsistency or mismatch
    TypeInconsistency,
    /// Shadowed variable
    ShadowedVariable,
}

impl SemanticIssue {
    /// Creates a new semantic issue
    pub fn new(
        issue_type: SemanticIssueType,
        severity: Severity,
        message: String,
        line: Option<usize>,
    ) -> Self {
        Self {
            issue_type,
            severity,
            message,
            line,
        }
    }

    /// Converts the issue to a formatted string
    pub fn to_string(&self) -> String {
        let severity_str = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        };

        if let Some(line) = self.line {
            format!("[{}] Line {}: {}", severity_str, line, self.message)
        } else {
            format!("[{}] {}", severity_str, self.message)
        }
    }
}
