// projects/libraries/ast_core/src/ast_path.rs
use std::fmt;

use crate::PathItem;

/// A path to a location in the AST.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AstPath(pub Vec<PathItem>);

impl fmt::Display for AstPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            match item {
                PathItem::Key(k) => write!(f, "{}", k)?,
                PathItem::Index(idx) => write!(f, "[{}]", idx)?,
            }
        }
        Ok(())
    }
}
