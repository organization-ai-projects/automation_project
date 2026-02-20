// projects/libraries/ast_core/src/ast_error_kind.rs
// The kind of validation error.
#[derive(Clone, Debug, PartialEq)]
pub enum AstErrorKind {
    MaxDepth {
        max: usize,
        got: usize,
    },
    MaxSize {
        kind: &'static str,
        max: usize,
    },
    DuplicateKey {
        key: String,
    },
    TypeMismatch {
        expected: &'static str,
        got: &'static str,
    },
}
