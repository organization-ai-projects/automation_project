// projects/libraries/symbolic/src/validator/type_visitor.rs
use std::collections::HashMap;
use syn::visit::Visit;
use syn::{Expr, Local, Pat, Stmt, Type};

/// Visitor to detect type inconsistencies
pub struct TypeVisitor {
    /// Map of variable names to their declared types
    pub variable_types: HashMap<String, String>,
    /// Detected type inconsistencies with statement index
    pub inconsistencies: Vec<(String, usize)>,
    /// Current statement index
    current_stmt_index: usize,
}

impl TypeVisitor {
    pub fn new() -> Self {
        Self {
            variable_types: HashMap::new(),
            inconsistencies: Vec::new(),
            current_stmt_index: 0,
        }
    }

    /// Extract variable name from pattern
    fn extract_var_name(&self, pat: &Pat) -> Option<String> {
        match pat {
            Pat::Ident(pat_ident) => Some(pat_ident.ident.to_string()),
            Pat::Type(pat_type) => self.extract_var_name(&pat_type.pat),
            _ => None,
        }
    }

    /// Extract type as string
    fn type_to_string(&self, ty: &Type) -> String {
        match ty {
            Type::Path(type_path) => {
                type_path
                    .path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::")
            }
            Type::Reference(type_ref) => {
                format!("&{}", self.type_to_string(&type_ref.elem))
            }
            Type::Tuple(type_tuple) => {
                let elems: Vec<String> = type_tuple
                    .elems
                    .iter()
                    .map(|t| self.type_to_string(t))
                    .collect();
                format!("({})", elems.join(", "))
            }
            _ => "unknown".to_string(),
        }
    }

    /// Check for type inconsistencies in assignment
    fn check_assignment(&mut self, var_name: &str, expr: &Expr) {
        if let Some(declared_type) = self.variable_types.get(var_name) {
            let inferred_type = self.infer_type_from_expr(expr);
            
            // Check for obvious mismatches
            if !inferred_type.is_empty() 
                && inferred_type != "unknown" 
                && declared_type != &inferred_type
                && !self.are_compatible_types(declared_type, &inferred_type) {
                let msg = format!(
                    "Type mismatch for '{}': expected '{}' but found '{}'",
                    var_name, declared_type, inferred_type
                );
                self.inconsistencies.push((msg, self.current_stmt_index));
            }
        }
    }

    /// Infer type from expression
    fn infer_type_from_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Str(_) => "String".to_string(),
                syn::Lit::Int(_) => "i32".to_string(),
                syn::Lit::Float(_) => "f64".to_string(),
                syn::Lit::Bool(_) => "bool".to_string(),
                syn::Lit::Char(_) => "char".to_string(),
                _ => "unknown".to_string(),
            },
            Expr::Path(expr_path) => {
                if let Some(ident) = expr_path.path.get_ident() {
                    let ident_str = ident.to_string();
                    // Check if it's a known variable
                    if let Some(var_type) = self.variable_types.get(&ident_str) {
                        return var_type.clone();
                    }
                }
                "unknown".to_string()
            }
            Expr::Call(expr_call) => {
                // Try to infer from function name
                if let Expr::Path(path) = expr_call.func.as_ref() {
                    if let Some(segment) = path.path.segments.last() {
                        let func_name = segment.ident.to_string();
                        // Common constructors
                        if func_name == "String" || func_name == "to_string" {
                            return "String".to_string();
                        }
                        if func_name == "Vec" {
                            return "Vec".to_string();
                        }
                    }
                }
                "unknown".to_string()
            }
            Expr::MethodCall(method_call) => {
                let method_name = method_call.method.to_string();
                if method_name == "to_string" || method_name == "into" {
                    return "String".to_string();
                }
                "unknown".to_string()
            }
            _ => "unknown".to_string(),
        }
    }

    /// Check if two types are compatible (handles some common cases)
    fn are_compatible_types(&self, declared: &str, inferred: &str) -> bool {
        // Exact match
        if declared == inferred {
            return true;
        }

        // String and &str are compatible
        if (declared == "String" && inferred == "&str")
            || (declared == "&str" && inferred == "String")
        {
            return true;
        }

        // Integer types can be compatible
        let int_types = ["i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize"];
        if int_types.contains(&declared) && int_types.contains(&inferred) {
            return true;
        }

        // Float types are compatible
        if (declared == "f32" || declared == "f64") && (inferred == "f32" || inferred == "f64") {
            return true;
        }

        false
    }
}

impl<'ast> Visit<'ast> for TypeVisitor {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        self.current_stmt_index += 1;

        if let Stmt::Local(local) = stmt {
            self.visit_local(local);
        }
        
        syn::visit::visit_stmt(self, stmt);
    }

    fn visit_local(&mut self, local: &'ast Local) {
        // Extract variable name and type
        if let Some(var_name) = self.extract_var_name(&local.pat) {
            // Skip variables starting with underscore
            if var_name.starts_with('_') {
                return;
            }

            // Get declared type if explicitly specified
            if let Pat::Type(pat_type) = &local.pat {
                let type_str = self.type_to_string(&pat_type.ty);
                self.variable_types.insert(var_name.clone(), type_str);
            } else if let Some(init) = &local.init {
                // Infer type from initialization
                let inferred_type = self.infer_type_from_expr(&init.expr);
                if !inferred_type.is_empty() && inferred_type != "unknown" {
                    self.variable_types.insert(var_name.clone(), inferred_type);
                }
            }

            // Check for type mismatch if both declared and initialized
            if let Pat::Type(pat_type) = &local.pat {
                if let Some(init) = &local.init {
                    let declared_type = self.type_to_string(&pat_type.ty);
                    let inferred_type = self.infer_type_from_expr(&init.expr);
                    
                    if !inferred_type.is_empty() 
                        && inferred_type != "unknown" 
                        && declared_type != inferred_type
                        && !self.are_compatible_types(&declared_type, &inferred_type) {
                        let msg = format!(
                            "Type mismatch in declaration of '{}': declared as '{}' but initialized with '{}'",
                            var_name, declared_type, inferred_type
                        );
                        self.inconsistencies.push((msg, self.current_stmt_index));
                    }
                }
            }
        }

        syn::visit::visit_local(self, local);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // Check for assignments
        if let Expr::Assign(assign) = expr {
            if let Expr::Path(path) = assign.left.as_ref() {
                if let Some(ident) = path.path.get_ident() {
                    let var_name = ident.to_string();
                    self.check_assignment(&var_name, &assign.right);
                }
            }
        }

        syn::visit::visit_expr(self, expr);
    }
}
