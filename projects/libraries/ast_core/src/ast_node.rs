// projects/libraries/ast_core/src/ast_node.rs
use crate::OpaqueValue;
use crate::{
    AstErrorKind, AstKey, AstKind, AstMeta, AstPath, AstValidationError, Number, Origin,
    ValidateLimits, ast_span::AstSpan, walk_validate,
};

use std::mem;

/// Macro to generate transformation methods for `AstNode`.
macro_rules! generate_transform_methods {
    ($($method:ident => $order:ident),* $(,)?) => {
        $(
            /// Applies a transformation `$order`.
            pub fn $method<F>(&self, f: &F) -> Self
            where
                F: Fn(&AstNode) -> AstNode,
            {
                match stringify!($order) {
                    "top_down" => {
                        let transformed = f(self);
                        transformed
                    }
                    "bottom_up" => {
                        f(self)
                    }
                    _ => self.clone(),
                }
            }
        )*
    };
}

/// Macro to generate validation methods for `AstNode`.
macro_rules! generate_validation_methods {
    ($($method:ident => $rule:expr),* $(,)?) => {
        $(
            /// Validates the AST node with the `$rule` rule.
            pub fn $method(&self) -> Result<(), AstValidationError> {
                $rule(self)
            }
        )*
    };
}

/// An AST node with metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct AstNode {
    pub kind: AstKind,
    pub meta: AstMeta,
}

impl Drop for AstNode {
    fn drop(&mut self) {
        let mut stack: Vec<AstKind> = Vec::new();
        let root_kind = mem::replace(&mut self.kind, AstKind::Null);
        stack.push(root_kind);

        while let Some(kind) = stack.pop() {
            match kind {
                AstKind::Array(mut items) => {
                    for mut item in items.drain(..) {
                        let child_kind = mem::replace(&mut item.kind, AstKind::Null);
                        stack.push(child_kind);
                    }
                }
                AstKind::Object(mut fields) => {
                    for (_, mut item) in fields.drain(..) {
                        let child_kind = mem::replace(&mut item.kind, AstKind::Null);
                        stack.push(child_kind);
                    }
                }
                AstKind::Null
                | AstKind::Bool(_)
                | AstKind::Number(_)
                | AstKind::String(_)
                | AstKind::Opaque(_) => {}
            }
        }
    }
}

impl AstNode {
    /// Creates a new node with the given kind and default metadata.
    pub fn new(kind: AstKind) -> Self {
        Self {
            kind,
            meta: AstMeta::default(),
        }
    }

    /// Sets the metadata on this node.
    pub fn with_meta(mut self, meta: AstMeta) -> Self {
        self.meta = meta;
        self
    }

    /// Sets the span on this node.
    pub fn with_span(mut self, span: AstSpan) -> Self {
        self.meta.span = Some(AstSpan::new(span.start, span.end));
        self
    }

    /// Sets the origin on this node.
    pub fn with_origin(mut self, origin: Origin) -> Self {
        self.meta.origin = Some(origin);
        self
    }

    /// Validates the AST with default limits.
    pub fn validate(&self) -> Result<(), AstValidationError> {
        self.validate_with(&ValidateLimits::default())
    }

    /// Validates the AST with custom limits.
    pub fn validate_with(&self, limits: &ValidateLimits) -> Result<(), AstValidationError> {
        walk_validate::validate_iterative(self, limits)
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Returns true if this node matches the given `AstKind`.
    fn matches_kind(&self, kind: &AstKind) -> bool {
        &self.kind == kind
    }

    /// Returns true if this node is null.
    pub fn is_null(&self) -> bool {
        self.matches_kind(&AstKind::Null)
    }

    /// Returns true if this node is a boolean.
    pub fn is_bool(&self) -> bool {
        matches!(self.kind, AstKind::Bool(_))
    }

    /// Returns true if this node is a number.
    pub fn is_number(&self) -> bool {
        matches!(self.kind, AstKind::Number(_))
    }

    /// Returns true if this node is a string.
    pub fn is_string(&self) -> bool {
        matches!(self.kind, AstKind::String(_))
    }

    /// Returns true if this node is an array.
    pub fn is_array(&self) -> bool {
        matches!(self.kind, AstKind::Array(_))
    }

    /// Returns true if this node is an object.
    pub fn is_object(&self) -> bool {
        matches!(self.kind, AstKind::Object(_))
    }

    /// Returns true if this node is opaque.
    pub fn is_opaque(&self) -> bool {
        matches!(self.kind, AstKind::Opaque(_))
    }

    /// Returns the value if this node is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self.kind {
            AstKind::Bool(b) => Some(b),
            _ => None,
        }
    }
    /// Returns the value if this node is a number.
    pub fn as_number(&self) -> Option<&Number> {
        match &self.kind {
            AstKind::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the value if this node is a string.
    pub fn as_string(&self) -> Option<&str> {
        match &self.kind {
            AstKind::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Returns the value if this node is an array.
    pub fn as_array(&self) -> Option<&[AstNode]> {
        match &self.kind {
            AstKind::Array(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    /// Returns the value if this node is an object.
    pub fn as_object(&self) -> Option<&[(AstKey, AstNode)]> {
        match &self.kind {
            AstKind::Object(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    /// Returns the value if this node is opaque.
    pub fn as_opaque(&self) -> Option<&OpaqueValue> {
        match &self.kind {
            AstKind::Opaque(v) => Some(v),
            _ => None,
        }
    }

    // Generate transformation methods using the macro.
    generate_transform_methods! {
        transform_top_down => top_down,
        transform_bottom_up => bottom_up,
    }

    // Generate validation methods using the macro.
    generate_validation_methods! {
        validate_non_empty => |node: &AstNode| {
            if !node.is_null() {
                Ok(())
            } else {
                Err(AstValidationError {
                    path: AstPath::default(),
                    kind: AstErrorKind::MaxSize { kind: "Node", max: 0 },
                })
            }
        },
        validate_has_children => |node: &AstNode| {
            if node.depth() > 1 {
                Ok(())
            } else {
                Err(AstValidationError {
                    path: AstPath::default(),
                    kind: AstErrorKind::MaxDepth { max: 1, got: 0 },
                })
            }
        },
    }

    /// Gets a field from an object by key.
    pub fn get(&self, key: &str) -> Option<&AstNode> {
        match &self.kind {
            AstKind::Object(fields) => fields
                .iter()
                .find(|(k, _)| k.as_str() == key)
                .map(|(_, v)| v),
            _ => None,
        }
    }

    /// Gets an element from an array by index.
    pub fn get_index(&self, index: usize) -> Option<&AstNode> {
        match &self.kind {
            AstKind::Array(items) => items.get(index),
            _ => None,
        }
    }

    // ========================================================================
    // Transformation
    // ========================================================================

    /// Applies a function to the node and its children recursively (post-rewrite).
    ///
    /// # Behavior
    /// This method applies the function `f` to the current node first, producing
    /// a rewritten node. It then recursively applies `f` to the children of the
    /// rewritten node. If `f` replaces the node with a leaf, the traversal stops.
    ///
    /// This is a top-down transformation. For a bottom-up approach, see
    /// `transform_bottom_up` (if implemented).
    pub fn transform<F>(&self, f: &F) -> Self
    where
        F: Fn(&AstNode) -> AstNode,
    {
        let mut transformed = f(self);
        let kind = match mem::replace(&mut transformed.kind, AstKind::Null) {
            AstKind::Array(items) => {
                AstKind::Array(items.iter().map(|item| item.transform(f)).collect())
            }
            AstKind::Object(fields) => AstKind::Object(
                fields
                    .iter()
                    .map(|(k, v)| (k.clone(), v.transform(f)))
                    .collect(),
            ),
            other => other,
        };
        let meta = std::mem::take(&mut transformed.meta);
        AstNode { kind, meta }
    }

    /// Visits all nodes recursively (depth-first), calling the visitor function.
    pub fn visit<F>(&self, f: &mut F)
    where
        F: FnMut(&AstNode),
    {
        f(self);
        match &self.kind {
            AstKind::Array(items) => {
                for item in items {
                    item.visit(f);
                }
            }
            AstKind::Object(fields) => {
                for (_, value) in fields {
                    value.visit(f);
                }
            }
            _ => {}
        }
    }

    /// Counts the total number of nodes in the AST.
    pub fn node_count(&self) -> usize {
        self.count_nodes(|_| true)
    }

    /// Counts the total number of nodes in the AST recursively.
    pub fn node_count_recursive(&self) -> usize {
        self.count_nodes(|_| true)
    }

    /// Helper function to count nodes based on a condition.
    fn count_nodes<F>(&self, condition: F) -> usize
    where
        F: Fn(&AstNode) -> bool,
    {
        let mut count = 0;
        self.visit(&mut |node| {
            if condition(node) {
                count += 1;
            }
        });
        count
    }

    /// Returns the maximum depth of the AST.
    pub fn depth(&self) -> usize {
        match &self.kind {
            AstKind::Array(items) => 1 + items.iter().map(|item| item.depth()).max().unwrap_or(0),
            AstKind::Object(fields) => 1 + fields.iter().map(|(_, v)| v.depth()).max().unwrap_or(0),
            _ => 1,
        }
    }

    pub fn validate_iterative(&self, limits: &ValidateLimits) -> Result<(), AstValidationError> {
        walk_validate::validate_iterative(self, limits)
    }

    /// Returns the name of the current kind as a string.
    pub fn kind_name(&self) -> &'static str {
        match self.kind {
            AstKind::Null => "null",
            AstKind::Bool(_) => "bool",
            AstKind::Number(_) => "number",
            AstKind::String(_) => "string",
            AstKind::Array(_) => "array",
            AstKind::Object(_) => "object",
            AstKind::Opaque(_) => "opaque",
        }
    }

    /// Tries to get the boolean value, or returns an error if the type is mismatched.
    pub fn try_bool(&self) -> Result<bool, AstErrorKind> {
        self.as_bool().ok_or(AstErrorKind::TypeMismatch {
            expected: "bool",
            got: self.kind_name(),
        })
    }

    /// Tries to get the number value, or returns an error if the type is mismatched.
    pub fn try_number(&self) -> Result<&Number, AstErrorKind> {
        self.as_number().ok_or(AstErrorKind::TypeMismatch {
            expected: "number",
            got: self.kind_name(),
        })
    }

    /// Tries to get the string value, or returns an error if the type is mismatched.
    pub fn try_string(&self) -> Result<&str, AstErrorKind> {
        self.as_string().ok_or(AstErrorKind::TypeMismatch {
            expected: "string",
            got: self.kind_name(),
        })
    }

    /// Tries to get the array value, or returns an error if the type is mismatched.
    pub fn try_array(&self) -> Result<&[AstNode], AstErrorKind> {
        self.as_array().ok_or(AstErrorKind::TypeMismatch {
            expected: "array",
            got: self.kind_name(),
        })
    }

    /// Tries to get the object value, or returns an error if the type is mismatched.
    pub fn try_object(&self) -> Result<&[(AstKey, AstNode)], AstErrorKind> {
        self.as_object().ok_or(AstErrorKind::TypeMismatch {
            expected: "object",
            got: self.kind_name(),
        })
    }

    /// Tries to get the opaque value, or returns an error if the type is mismatched.
    pub fn try_opaque(&self) -> Result<&OpaqueValue, AstErrorKind> {
        self.as_opaque().ok_or(AstErrorKind::TypeMismatch {
            expected: "opaque",
            got: self.kind_name(),
        })
    }
}
