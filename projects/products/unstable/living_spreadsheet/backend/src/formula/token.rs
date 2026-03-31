#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Text(String),
    CellRef(String),
    RangeRef(String, String),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Comma,
    Equals,
}
