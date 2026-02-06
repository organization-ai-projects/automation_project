// projects/libraries/symbolic/src/validator/import_visitor.rs
use std::collections::{HashMap, HashSet};
use syn::visit::Visit;
use syn::{Expr, Item, Type, UseTree};

/// Visitor to track imports and their usage
pub struct ImportVisitor {
    pub imports: HashMap<String, usize>,
    pub used_identifiers: HashSet<String>,
    current_line: usize,
}

impl ImportVisitor {
    pub fn new() -> Self {
        Self {
            imports: HashMap::new(),
            used_identifiers: HashSet::new(),
            current_line: 0,
        }
    }

    pub fn extract_use_names(&mut self, use_tree: &UseTree, line: usize) {
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
        // Track identifier usage in expressions
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

    fn visit_type(&mut self, ty: &'ast Type) {
        // Track identifier usage in type positions (e.g., function signatures, struct fields)
        if let Type::Path(type_path) = ty {
            if let Some(ident) = type_path.path.get_ident() {
                self.used_identifiers.insert(ident.to_string());
            }
            // Also track segments in type paths like std::collections::HashMap
            for segment in type_path.path.segments.iter() {
                self.used_identifiers.insert(segment.ident.to_string());
            }
        }
        syn::visit::visit_type(self, ty);
    }
}
