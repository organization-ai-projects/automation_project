// projects/libraries/symbolic/src/validator/type_visitor.rs
use std::collections::HashMap;
use syn::visit::Visit;
use syn::{Expr, Local, Pat, Stmt, Type};

/// Common integer types for compatibility checking
const INT_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
];

/// Visitor to detect type inconsistencies
pub struct TypeVisitor {
    /// Stack of scopes, each scope is a map of variable names to their declared types
    /// The last element is the current (innermost) scope
    scope_stack: Vec<HashMap<String, String>>,
    /// Detected type inconsistencies with statement index
    pub inconsistencies: Vec<(String, usize)>,
    /// Current statement index
    current_stmt_index: usize,
}

impl TypeVisitor {
    pub fn new() -> Self {
        Self {
            scope_stack: vec![HashMap::new()], // Start with one global scope
            inconsistencies: Vec::new(),
            current_stmt_index: 0,
        }
    }

    /// Get the type of a variable by searching from innermost to outermost scope
    fn get_variable_type(&self, var_name: &str) -> Option<String> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(ty) = scope.get(var_name) {
                return Some(ty.clone());
            }
        }
        None
    }

    /// Insert or update a variable type in the current scope
    fn set_variable_type(&mut self, var_name: String, ty: String) {
        if let Some(current_scope) = self.scope_stack.last_mut() {
            current_scope.insert(var_name, ty);
        }
    }

    /// Push a new scope onto the stack
    fn push_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }

    /// Pop the current scope from the stack
    fn pop_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
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
            Type::Path(type_path) => type_path
                .path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
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
        if let Some(declared_type) = self.get_variable_type(var_name) {
            let inferred_type = self.infer_type_from_expr(expr);

            // Check for obvious mismatches
            if !inferred_type.is_empty()
                && inferred_type != "unknown"
                && declared_type != "unknown"
                && declared_type != inferred_type
                && !self.are_compatible_types(&declared_type, &inferred_type)
            {
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
                syn::Lit::Str(_) => "&str".to_string(),
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
                    if let Some(var_type) = self.get_variable_type(&ident_str) {
                        return var_type;
                    }
                }
                "unknown".to_string()
            }
            Expr::Call(expr_call) => {
                // Try to infer from function name
                if let Expr::Path(func_path) = expr_call.func.as_ref()
                    && let Some(segment) = func_path.path.segments.last()
                {
                    let func_name = segment.ident.to_string();
                    // Common constructors
                    if func_name == "String" || func_name == "to_string" {
                        return "String".to_string();
                    }
                    if func_name == "Vec" {
                        return "Vec".to_string();
                    }
                }

                "unknown".to_string()
            }
            Expr::MethodCall(method_call) => {
                let method_name = method_call.method.to_string();
                if method_name == "to_string" {
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
        if INT_TYPES.contains(&declared) && INT_TYPES.contains(&inferred) {
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
                self.set_variable_type(var_name.clone(), type_str);
            } else if let Some(init) = &local.init {
                // Infer type from initialization
                let inferred_type = self.infer_type_from_expr(&init.expr);
                if !inferred_type.is_empty() && inferred_type != "unknown" {
                    self.set_variable_type(var_name.clone(), inferred_type);
                }
            }

            // Check for type mismatch if both declared and initialized
            if let (Pat::Type(pat_type), Some(init)) = (&local.pat, &local.init) {
                let declared_type = self.type_to_string(&pat_type.ty);
                let inferred_type = self.infer_type_from_expr(&init.expr);

                if !inferred_type.is_empty()
                    && inferred_type != "unknown"
                    && declared_type != "unknown"
                    && declared_type != inferred_type
                    && !self.are_compatible_types(&declared_type, &inferred_type)
                {
                    let msg = format!(
                        "Type mismatch in declaration of '{}': declared as '{}' but initialized with '{}'",
                        var_name, declared_type, inferred_type
                    );
                    self.inconsistencies.push((msg, self.current_stmt_index));
                }
            }
        }

        syn::visit::visit_local(self, local);
    }

    fn visit_block(&mut self, block: &'ast syn::Block) {
        // Push a new scope when entering a block
        self.push_scope();
        syn::visit::visit_block(self, block);
        // Pop the scope when exiting the block
        self.pop_scope();
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // Check for assignments
        if let Expr::Assign(assign_expr) = expr
            && let Expr::Path(left_path) = assign_expr.left.as_ref()
            && let Some(ident) = left_path.path.get_ident()
        {
            let var_name = ident.to_string();
            self.check_assignment(&var_name, &assign_expr.right);
        }

        syn::visit::visit_expr(self, expr);
    }
}
