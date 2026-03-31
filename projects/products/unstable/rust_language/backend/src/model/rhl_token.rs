use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RhlToken {
    KeywordFn,
    KeywordLet,
    KeywordMut,
    KeywordReturn,
    KeywordIf,
    KeywordElse,
    KeywordStruct,
    KeywordImpl,
    KeywordFor,
    KeywordWhile,
    KeywordPub,
    KeywordUse,
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Arrow,
    FatArrow,
    Colon,
    Semicolon,
    Comma,
    Dot,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Equals,
    DoubleEquals,
    NotEquals,
    Plus,
    Minus,
    Star,
    Slash,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Ampersand,
    Pipe,
    Bang,
    TypeI32,
    TypeI64,
    TypeF64,
    TypeBool,
    TypeString,
    Eof,
}

impl RhlToken {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            RhlToken::KeywordFn
                | RhlToken::KeywordLet
                | RhlToken::KeywordMut
                | RhlToken::KeywordReturn
                | RhlToken::KeywordIf
                | RhlToken::KeywordElse
                | RhlToken::KeywordStruct
                | RhlToken::KeywordImpl
                | RhlToken::KeywordFor
                | RhlToken::KeywordWhile
                | RhlToken::KeywordPub
                | RhlToken::KeywordUse
        )
    }
}
