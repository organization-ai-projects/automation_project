// projects/libraries/symbolic/src/validator/variable_visitor.rs
use regex::Regex;
use std::collections::{HashMap, HashSet};
use syn::visit::Visit;
use syn::{Expr, Stmt};

/// Visitor to track variable usage
pub struct VariableVisitor {
    pub declared_variables: HashMap<String, usize>,
    pub used_variables: HashSet<String>,
    regex_cache: HashMap<String, Regex>,
}

impl VariableVisitor {
    pub fn new(declared_variables: HashMap<String, usize>) -> Self {
        // Pre-compile regex patterns for all declared variables
        let mut regex_cache = HashMap::new();
        for var_name in declared_variables.keys() {
            let pattern = format!(r"\b{}\b", regex::escape(var_name));
            if let Ok(re) = Regex::new(&pattern) {
                regex_cache.insert(var_name.clone(), re);
            }
        }

        Self {
            declared_variables,
            used_variables: HashSet::new(),
            regex_cache,
        }
    }

    /// Check if variable is used in tokens using pre-compiled regex
    fn check_variable_usage_in_tokens(&mut self, tokens: &str) {
        for (var_name, regex) in &self.regex_cache {
            if regex.is_match(tokens) {
                self.used_variables.insert(var_name.clone());
            }
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
                self.check_variable_usage_in_tokens(&tokens);
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        // Also check for macro statements (like println!)
        if let Stmt::Macro(stmt_macro) = stmt {
            let tokens = stmt_macro.mac.tokens.to_string();
            self.check_variable_usage_in_tokens(&tokens);
        }
        syn::visit::visit_stmt(self, stmt);
    }
}
