//! projects/products/unstable/rust_language/backend/src/model/rhl_ast.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum RhlAst {
    Program(Vec<RhlAst>),
    FnDecl {
        name: String,
        params: Vec<(String, String)>,
        return_type: Option<String>,
        body: Vec<RhlAst>,
    },
    StructDecl {
        name: String,
        fields: Vec<(String, String)>,
    },
    LetBinding {
        name: String,
        mutable: bool,
        type_annotation: Option<String>,
        value: Box<RhlAst>,
    },
    Return(Box<RhlAst>),
    IfExpr {
        condition: Box<RhlAst>,
        then_body: Vec<RhlAst>,
        else_body: Option<Vec<RhlAst>>,
    },
    WhileLoop {
        condition: Box<RhlAst>,
        body: Vec<RhlAst>,
    },
    BinaryOp {
        left: Box<RhlAst>,
        op: String,
        right: Box<RhlAst>,
    },
    Call {
        callee: String,
        args: Vec<RhlAst>,
    },
    FieldAccess {
        object: Box<RhlAst>,
        field: String,
    },
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Block(Vec<RhlAst>),
    Assignment {
        target: String,
        value: Box<RhlAst>,
    },
}
