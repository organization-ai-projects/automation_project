// projects/libraries/layers/domain/symbolic/src/validator/semantic_analyzer.rs
use crate::validator::dead_code_visitor::DeadCodeVisitor;
use crate::validator::import_visitor::ImportVisitor;
use crate::validator::semantic_issue::{SemanticIssue, SemanticIssueType, Severity};
use crate::validator::type_visitor::TypeVisitor;
use crate::validator::variable_collector::VariableCollector;
use crate::validator::variable_visitor::VariableVisitor;
use syn::File;
use syn::visit::Visit;
use tracing;

/// Analyzer for semantic issues in Rust code
pub struct SemanticAnalyzer {
    strict_mode: bool,
}

impl SemanticAnalyzer {
    pub fn new(strict_mode: bool) -> Self {
        Self { strict_mode }
    }

    /// Analyzes a syntax tree for semantic issues
    pub fn analyze(&self, syntax_tree: &File) -> Vec<SemanticIssue> {
        let mut issues = Vec::new();

        // Check for unused variables
        issues.extend(self.check_unused_variables(syntax_tree));

        // Check for unused imports
        issues.extend(self.check_unused_imports(syntax_tree));

        // Check for dead code
        issues.extend(self.check_dead_code(syntax_tree));

        // Check for type inconsistencies
        issues.extend(self.check_type_inconsistencies(syntax_tree));

        issues
    }

    /// Checks for unused variables in the syntax tree
    fn check_unused_variables(&self, syntax_tree: &File) -> Vec<SemanticIssue> {
        let mut issues = Vec::new();

        // First pass: collect all declared variables
        let mut collector = VariableCollector::new();
        collector.visit_file(syntax_tree);

        // Second pass: check usage with the collected variables
        let mut visitor = VariableVisitor::new(collector.declared_variables);
        visitor.visit_file(syntax_tree);

        for (var_name, declared_line) in visitor.declared_variables.iter() {
            // Skip variables that start with underscore (convention for unused)
            if var_name.starts_with('_') {
                continue;
            }

            if !visitor.used_variables.contains(var_name) {
                let severity = if self.strict_mode {
                    Severity::Error
                } else {
                    Severity::Warning
                };

                issues.push(SemanticIssue::new(
                    SemanticIssueType::UnusedVariable,
                    severity,
                    format!("Variable '{}' is declared but never used", var_name),
                    Some(*declared_line),
                ));
            }
        }

        tracing::debug!("Found {} unused variable issues", issues.len());
        issues
    }

    /// Checks for unused imports in the syntax tree
    fn check_unused_imports(&self, syntax_tree: &File) -> Vec<SemanticIssue> {
        let mut issues = Vec::new();
        let mut visitor = ImportVisitor::new();
        visitor.visit_file(syntax_tree);

        for (import_name, line) in visitor.imports.iter() {
            // Skip common re-exports and glob imports
            if import_name == "*" || import_name == "self" {
                continue;
            }

            if !visitor.used_identifiers.contains(import_name) {
                let severity = if self.strict_mode {
                    Severity::Error
                } else {
                    Severity::Warning
                };

                issues.push(SemanticIssue::new(
                    SemanticIssueType::UnusedImport,
                    severity,
                    format!("Import '{}' is declared but never used", import_name),
                    Some(*line),
                ));
            }
        }

        tracing::debug!("Found {} unused import issues", issues.len());
        issues
    }

    /// Checks for dead code (unreachable code after return, break, continue)
    fn check_dead_code(&self, syntax_tree: &File) -> Vec<SemanticIssue> {
        let mut issues = Vec::new();
        let mut visitor = DeadCodeVisitor::new();
        visitor.visit_file(syntax_tree);

        for line in visitor.dead_code_lines.iter() {
            let severity = if self.strict_mode {
                Severity::Error
            } else {
                Severity::Warning
            };

            issues.push(SemanticIssue::new(
                SemanticIssueType::DeadCode,
                severity,
                "Unreachable code detected after control flow statement".to_string(),
                Some(*line),
            ));
        }

        tracing::debug!("Found {} dead code issues", issues.len());
        issues
    }

    /// Checks for type inconsistencies
    fn check_type_inconsistencies(&self, syntax_tree: &File) -> Vec<SemanticIssue> {
        let mut issues = Vec::new();
        let mut visitor = TypeVisitor::new();
        visitor.visit_file(syntax_tree);

        for (message, stmt_index) in visitor.inconsistencies.iter() {
            let severity = if self.strict_mode {
                Severity::Error
            } else {
                Severity::Warning
            };

            issues.push(SemanticIssue::new(
                SemanticIssueType::TypeInconsistency,
                severity,
                message.clone(),
                Some(*stmt_index),
            ));
        }

        tracing::debug!("Found {} type inconsistency issues", issues.len());
        issues
    }
}
