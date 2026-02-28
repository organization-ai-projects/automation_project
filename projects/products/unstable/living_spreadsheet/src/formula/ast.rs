use crate::model::cell_id::CellId;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Text(String),
    CellRef(CellId),
    RangeRef(CellId, CellId),
    BinOp { op: BinOpKind, lhs: Box<Expr>, rhs: Box<Expr> },
    FunctionCall { name: String, args: Vec<Expr> },
    Neg(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}
