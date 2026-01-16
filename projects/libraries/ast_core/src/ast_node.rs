use std::borrow::Cow;

use crate::{
    AstBuilder, AstKey, AstKind, AstMeta, AstValidationError, Number, Origin, Span, ValidateLimits,
};

/// An AST node with metadata.
#[derive(Clone, Debug, PartialEq)]
pub struct AstNode {
    pub kind: AstKind,
    pub meta: AstMeta,
}

impl Drop for AstNode {
    fn drop(&mut self) {
        let mut stack: Vec<AstKind> = Vec::new();
        let root_kind = std::mem::replace(&mut self.kind, AstKind::Null);
        stack.push(root_kind);

        while let Some(kind) = stack.pop() {
            match kind {
                AstKind::Array(mut items) => {
                    for mut item in items.drain(..) {
                        let child_kind = std::mem::replace(&mut item.kind, AstKind::Null);
                        stack.push(child_kind);
                    }
                }
                AstKind::Object(mut fields) => {
                    for (_, mut item) in fields.drain(..) {
                        let child_kind = std::mem::replace(&mut item.kind, AstKind::Null);
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
    pub fn with_span(mut self, start: u32, end: u32) -> Self {
        self.meta.span = Some(Span { start, end });
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
        crate::walk_validate::validate_iterative(self, limits)
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Returns true if this node is null.
    pub fn is_null(&self) -> bool {
        matches!(self.kind, AstKind::Null)
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

    /// Returns the boolean value if this node is a Bool.
    pub fn as_bool(&self) -> Option<bool> {
        match &self.kind {
            AstKind::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the number if this node is a Number.
    pub fn as_number(&self) -> Option<&Number> {
        match &self.kind {
            AstKind::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the string if this node is a String.
    pub fn as_str(&self) -> Option<&str> {
        match &self.kind {
            AstKind::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the array if this node is an Array.
    pub fn as_array(&self) -> Option<&[AstNode]> {
        match &self.kind {
            AstKind::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns the object fields if this node is an Object.
    pub fn as_object(&self) -> Option<&[(AstKey, AstNode)]> {
        match &self.kind {
            AstKind::Object(obj) => Some(obj),
            _ => None,
        }
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
        let kind = match std::mem::replace(&mut transformed.kind, AstKind::Null) {
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

    /// Applies a function to all nodes recursively (bottom-up).
    ///
    /// # Behavior
    /// This method first applies the function `f` to all children of the node,
    /// and then applies `f` to the current node itself. This is a bottom-up
    /// transformation, useful for cases where child transformations must be
    /// completed before transforming the parent.
    pub fn transform_bottom_up<F>(&self, f: &F) -> Self
    where
        F: Fn(&AstNode) -> AstNode,
    {
        let kind = match &self.kind {
            AstKind::Array(items) => AstKind::Array(
                items
                    .iter()
                    .map(|item| item.transform_bottom_up(f))
                    .collect(),
            ),
            AstKind::Object(fields) => AstKind::Object(
                fields
                    .iter()
                    .map(|(k, v)| (k.clone(), v.transform_bottom_up(f)))
                    .collect(),
            ),
            other => other.clone(),
        };
        let transformed = AstNode {
            kind,
            meta: self.meta.clone(),
        };
        f(&transformed)
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
        let mut count = 0;
        self.visit(&mut |_| count += 1);
        count
    }

    /// Counts the total number of nodes in the AST recursively.
    pub fn node_count_recursive(&self) -> usize {
        match &self.kind {
            AstKind::Array(items) => {
                1 + items
                    .iter()
                    .map(|item| item.node_count_recursive())
                    .sum::<usize>()
            }
            AstKind::Object(fields) => {
                1 + fields
                    .iter()
                    .map(|(_, v)| v.node_count_recursive())
                    .sum::<usize>()
            }
            _ => 1,
        }
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
        crate::walk_validate::validate_iterative(self, limits)
    }
}

// ============================================================================
// From implementations for AstNode
// ============================================================================

impl From<()> for AstNode {
    fn from(_: ()) -> Self {
        AstBuilder::null()
    }
}

impl From<bool> for AstNode {
    fn from(value: bool) -> Self {
        AstBuilder::bool(value)
    }
}

impl From<i8> for AstNode {
    fn from(value: i8) -> Self {
        AstBuilder::int(value as i64)
    }
}

impl From<i16> for AstNode {
    fn from(value: i16) -> Self {
        AstBuilder::int(value as i64)
    }
}

impl From<i32> for AstNode {
    fn from(value: i32) -> Self {
        AstBuilder::int(value as i64)
    }
}

impl From<i64> for AstNode {
    fn from(value: i64) -> Self {
        AstBuilder::int(value)
    }
}

impl From<u8> for AstNode {
    fn from(value: u8) -> Self {
        AstBuilder::uint(value as u64)
    }
}

impl From<u16> for AstNode {
    fn from(value: u16) -> Self {
        AstBuilder::uint(value as u64)
    }
}

impl From<u32> for AstNode {
    fn from(value: u32) -> Self {
        AstBuilder::uint(value as u64)
    }
}

impl From<u64> for AstNode {
    fn from(value: u64) -> Self {
        AstBuilder::uint(value)
    }
}

impl From<f32> for AstNode {
    fn from(value: f32) -> Self {
        AstBuilder::float(value as f64)
    }
}

impl From<f64> for AstNode {
    fn from(value: f64) -> Self {
        AstBuilder::float(value)
    }
}

impl From<&str> for AstNode {
    fn from(value: &str) -> Self {
        AstBuilder::string(value)
    }
}

impl From<String> for AstNode {
    fn from(value: String) -> Self {
        AstBuilder::string(value)
    }
}

impl From<&String> for AstNode {
    fn from(value: &String) -> Self {
        AstBuilder::string(value.as_str())
    }
}

impl<'a> From<Cow<'a, str>> for AstNode {
    fn from(value: Cow<'a, str>) -> Self {
        AstBuilder::string(value.into_owned())
    }
}

impl From<isize> for AstNode {
    fn from(value: isize) -> Self {
        if value >= 0 {
            AstBuilder::uint(value as u64)
        } else {
            AstBuilder::int(value as i64)
        }
    }
}

impl From<usize> for AstNode {
    fn from(value: usize) -> Self {
        if value <= u64::MAX as usize {
            AstBuilder::uint(value as u64)
        } else {
            AstBuilder::string(value.to_string())
        }
    }
}

impl From<i128> for AstNode {
    fn from(value: i128) -> Self {
        if value >= i64::MIN as i128 && value <= i64::MAX as i128 {
            AstBuilder::int(value as i64)
        } else {
            AstBuilder::string(value.to_string())
        }
    }
}

impl From<u128> for AstNode {
    fn from(value: u128) -> Self {
        if value <= u64::MAX as u128 {
            AstBuilder::uint(value as u64)
        } else {
            AstBuilder::string(value.to_string())
        }
    }
}
