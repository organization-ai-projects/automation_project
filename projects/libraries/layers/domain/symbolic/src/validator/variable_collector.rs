// projects/libraries/layers/domain/symbolic/src/validator/variable_collector.rs
use std::collections::HashMap;
use syn::visit::Visit;
use syn::{Local, Pat, Stmt};

/// Visitor to collect variable declarations
pub struct VariableCollector {
    pub declared_variables: HashMap<String, usize>,
    current_line: usize,
}

impl VariableCollector {
    pub fn new() -> Self {
        Self {
            declared_variables: HashMap::new(),
            current_line: 0,
        }
    }

    /// Recursively collect all identifier bindings from a pattern
    fn collect_idents_from_pat(&mut self, pat: &Pat) {
        match pat {
            Pat::Ident(pat_ident) => {
                let var_name = pat_ident.ident.to_string();
                self.declared_variables.insert(var_name, self.current_line);
            }
            Pat::Tuple(pat_tuple) => {
                for elem in &pat_tuple.elems {
                    self.collect_idents_from_pat(elem);
                }
            }
            Pat::Struct(pat_struct) => {
                for field in &pat_struct.fields {
                    self.collect_idents_from_pat(&field.pat);
                }
            }
            Pat::TupleStruct(pat_tuple_struct) => {
                for elem in &pat_tuple_struct.elems {
                    self.collect_idents_from_pat(elem);
                }
            }
            Pat::Slice(pat_slice) => {
                for elem in &pat_slice.elems {
                    self.collect_idents_from_pat(elem);
                }
            }
            Pat::Reference(pat_ref) => {
                self.collect_idents_from_pat(&pat_ref.pat);
            }
            Pat::Type(pat_type) => {
                self.collect_idents_from_pat(&pat_type.pat);
            }
            Pat::Paren(pat_paren) => {
                self.collect_idents_from_pat(&pat_paren.pat);
            }
            Pat::Or(pat_or) => {
                // All cases must bind the same identifiers; collect from the first
                if let Some(first_case) = pat_or.cases.first() {
                    self.collect_idents_from_pat(first_case);
                }
            }
            // Patterns that do not introduce new bindings are ignored
            _ => {}
        }
    }
}

impl<'ast> Visit<'ast> for VariableCollector {
    fn visit_local(&mut self, local: &'ast Local) {
        // Extract variable names from patterns, including destructuring
        self.collect_idents_from_pat(&local.pat);
        syn::visit::visit_local(self, local);
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        self.current_line += 1;
        syn::visit::visit_stmt(self, stmt);
    }
}
