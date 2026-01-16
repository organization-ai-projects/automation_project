use std::collections::{BTreeMap, BTreeSet};

use crate::{ExtId, Origin, Span};

/// Metadata attached to AST nodes.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AstMeta {
    /// Source span (byte offsets)
    pub span: Option<Span>,
    /// Origin of this node
    pub origin: Option<Origin>,
    /// Boolean flags
    pub flags: BTreeSet<&'static str>,
    /// String attributes
    pub attrs: BTreeMap<&'static str, String>,
    /// Extension data keyed by ExtId
    pub ext: BTreeMap<ExtId, Vec<u8>>,
}
