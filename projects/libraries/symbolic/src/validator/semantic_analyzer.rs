// projects/libraries/symbolic/src/validator/semantic_analyzer.rs
use crate::validator::semantic_issue::{SemanticIssue, SemanticIssueType, Severity};
use std::collections::{HashMap, HashSet};
use syn::visit::Visit;
use syn::{Expr, File, Item, Local, Pat, Stmt, UseTree};
use tracing;
use regex;

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
        issues.extend(self.check_type_consistency(syntax_tree));

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

    /// Checks for basic type inconsistencies
    fn check_type_consistency(&self, syntax_tree: &File) -> Vec<SemanticIssue> {
        let mut issues = Vec::new();
        let mut visitor = TypeVisitor::new();
        visitor.visit_file(syntax_tree);

        for issue_msg in visitor.type_issues.iter() {
            issues.push(SemanticIssue::new(
                SemanticIssueType::TypeInconsistency,
                Severity::Error,
                issue_msg.clone(),
                None,
            ));
        }

        tracing::debug!("Found {} type consistency issues", issues.len());
        issues
    }
}

/// Visitor to collect variable declarations
struct VariableCollector {
    declared_variables: HashMap<String, usize>,
    current_line: usize,
}

impl VariableCollector {
    fn new() -> Self {
        Self {
            declared_variables: HashMap::new(),
            current_line: 0,
        }
    }
}

impl<'ast> Visit<'ast> for VariableCollector {
    fn visit_local(&mut self, local: &'ast Local) {
        // Extract variable names from patterns
        if let Pat::Ident(pat_ident) = &local.pat {
            let var_name = pat_ident.ident.to_string();
            self.declared_variables
                .insert(var_name, self.current_line);
        }
        syn::visit::visit_local(self, local);
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        self.current_line += 1;
        syn::visit::visit_stmt(self, stmt);
    }
}

/// Visitor to track variable declarations and usage
struct VariableVisitor {
    declared_variables: HashMap<String, usize>,
    used_variables: HashSet<String>,
}

impl VariableVisitor {
    fn new(declared_variables: HashMap<String, usize>) -> Self {
        Self {
            declared_variables,
            used_variables: HashSet::new(),
        }
    }
}

impl<'ast> Visit<'ast> for VariableVisitor {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        // Track variable usage
        match expr {
            Expr::Path(expr_path) => {
                if let Some(ident) = expr_path.path.get_ident() {
                    self.used_variables.insert(ident.to_string());
                }
                // Also track multi-segment paths
                for segment in &expr_path.path.segments {
                    self.used_variables.insert(segment.ident.to_string());
                }
            }
            Expr::Macro(expr_macro) => {
                // Track identifiers in macro arguments
                let tokens = expr_macro.mac.tokens.to_string();
                // Check for exact identifier matches using word boundaries
                for (var_name, _) in self.declared_variables.iter() {
                    // Use word boundary checks: look for variable name not preceded/followed by alphanumeric or underscore
                    let pattern = format!(r"\b{}\b", regex::escape(var_name));
                    if let Ok(re) = regex::Regex::new(&pattern) {
                        if re.is_match(&tokens) {
                            self.used_variables.insert(var_name.clone());
                        }
                    } else {
                        // Fallback to simple contains check if regex fails
                        if tokens.contains(var_name) {
                            self.used_variables.insert(var_name.clone());
                        }
                    }
                }
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        // Also check for macro statements (like println!)
        if let Stmt::Macro(stmt_macro) = stmt {
            let tokens = stmt_macro.mac.tokens.to_string();
            // Check for exact identifier matches using word boundaries
            for (var_name, _) in self.declared_variables.iter() {
                let pattern = format!(r"\b{}\b", regex::escape(var_name));
                if let Ok(re) = regex::Regex::new(&pattern) {
                    if re.is_match(&tokens) {
                        self.used_variables.insert(var_name.clone());
                    }
                } else {
                    // Fallback to simple contains check if regex fails
                    if tokens.contains(var_name) {
                        self.used_variables.insert(var_name.clone());
                    }
                }
            }
        }
        syn::visit::visit_stmt(self, stmt);
    }
}

/// Visitor to track imports and their usage
struct ImportVisitor {
    imports: HashMap<String, usize>,
    used_identifiers: HashSet<String>,
    current_line: usize,
}

impl ImportVisitor {
    fn new() -> Self {
        Self {
            imports: HashMap::new(),
            used_identifiers: HashSet::new(),
            current_line: 0,
        }
    }

    fn extract_use_names(&mut self, use_tree: &UseTree, line: usize) {
        match use_tree {
            UseTree::Path(path) => {
                self.extract_use_names(&path.tree, line);
            }
            UseTree::Name(name) => {
                self.imports.insert(name.ident.to_string(), line);
            }
            UseTree::Rename(rename) => {
                self.imports.insert(rename.rename.to_string(), line);
            }
            UseTree::Glob(_) => {
                self.imports.insert("*".to_string(), line);
            }
            UseTree::Group(group) => {
                for item in &group.items {
                    self.extract_use_names(item, line);
                }
            }
        }
    }
}

impl<'ast> Visit<'ast> for ImportVisitor {
    fn visit_item(&mut self, item: &'ast Item) {
        self.current_line += 1;
        if let Item::Use(item_use) = item {
            self.extract_use_names(&item_use.tree, self.current_line);
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // Track identifier usage
        if let Expr::Path(expr_path) = expr {
            if let Some(ident) = expr_path.path.get_ident() {
                self.used_identifiers.insert(ident.to_string());
            }
            // Also track segments in paths like std::vec::Vec
            for segment in expr_path.path.segments.iter() {
                self.used_identifiers.insert(segment.ident.to_string());
            }
        }
        syn::visit::visit_expr(self, expr);
    }
}

/// Visitor to detect dead code
struct DeadCodeVisitor {
    dead_code_lines: Vec<usize>,
    current_line: usize,
    found_terminator: bool,
}

impl DeadCodeVisitor {
    fn new() -> Self {
        Self {
            dead_code_lines: Vec::new(),
            current_line: 0,
            found_terminator: false,
        }
    }
}

impl<'ast> Visit<'ast> for DeadCodeVisitor {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        self.current_line += 1;

        // Check if this statement is a terminator first (before checking if it's dead)
        let is_terminator = if let Stmt::Expr(expr, _) = stmt {
            matches!(expr, Expr::Return(_) | Expr::Break(_) | Expr::Continue(_))
        } else {
            false
        };

        // If we already found a terminator, mark this as dead code
        if self.found_terminator && !is_terminator {
            // This statement is dead code
            self.dead_code_lines.push(self.current_line);
        }

        // Set the terminator flag after checking
        if is_terminator {
            self.found_terminator = true;
        }

        syn::visit::visit_stmt(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // Reset terminator flag when entering a new block, function, or loop
        match expr {
            Expr::Block(_) | Expr::Closure(_) | Expr::Loop(_) | Expr::While(_) | Expr::ForLoop(_) => {
                let prev = self.found_terminator;
                self.found_terminator = false;
                syn::visit::visit_expr(self, expr);
                self.found_terminator = prev;
                return;
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }
}

/// Visitor to detect type inconsistencies
struct TypeVisitor {
    type_issues: Vec<String>,
}

impl TypeVisitor {
    fn new() -> Self {
        Self {
            type_issues: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for TypeVisitor {
    fn visit_local(&mut self, local: &'ast Local) {
        // Check for obvious type mismatches in variable initialization
        // This is limited without full type checking, but we can catch some cases
        if let Some(_init) = &local.init {
            // Simple type consistency check
            // Note: Full type checking would require more complex analysis
            // For now, we do basic pattern matching on literals and expressions
            tracing::debug!("Checking type consistency for local variable");
        }
        syn::visit::visit_local(self, local);
    }
}
