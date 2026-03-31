use crate::compiler::lexer::Lexer;
use crate::model::rhl_token::RhlToken;

#[test]
fn tokenize_empty_input() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![RhlToken::Eof]);
}

#[test]
fn tokenize_fn_keyword() {
    let mut lexer = Lexer::new("fn");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![RhlToken::KeywordFn, RhlToken::Eof]);
}

#[test]
fn tokenize_let_binding() {
    let mut lexer = Lexer::new("let x = 42;");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            RhlToken::KeywordLet,
            RhlToken::Identifier("x".into()),
            RhlToken::Equals,
            RhlToken::IntegerLiteral(42),
            RhlToken::Semicolon,
            RhlToken::Eof,
        ]
    );
}

#[test]
fn tokenize_string_literal() {
    let mut lexer = Lexer::new("\"hello world\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            RhlToken::StringLiteral("hello world".into()),
            RhlToken::Eof,
        ]
    );
}

#[test]
fn tokenize_float_literal() {
    let mut lexer = Lexer::new("3.14");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![RhlToken::FloatLiteral(3.14), RhlToken::Eof]
    );
}

#[test]
fn tokenize_comparison_operators() {
    let mut lexer = Lexer::new("== != <= >=");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            RhlToken::DoubleEquals,
            RhlToken::NotEquals,
            RhlToken::LessEqual,
            RhlToken::GreaterEqual,
            RhlToken::Eof,
        ]
    );
}

#[test]
fn tokenize_arrow() {
    let mut lexer = Lexer::new("-> =>");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![RhlToken::Arrow, RhlToken::FatArrow, RhlToken::Eof]
    );
}

#[test]
fn tokenize_type_keywords() {
    let mut lexer = Lexer::new("i32 i64 f64 bool String");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            RhlToken::TypeI32,
            RhlToken::TypeI64,
            RhlToken::TypeF64,
            RhlToken::TypeBool,
            RhlToken::TypeString,
            RhlToken::Eof,
        ]
    );
}

#[test]
fn tokenize_bool_literals() {
    let mut lexer = Lexer::new("true false");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            RhlToken::BoolLiteral(true),
            RhlToken::BoolLiteral(false),
            RhlToken::Eof,
        ]
    );
}

#[test]
fn tokenize_comment_skipped() {
    let mut lexer = Lexer::new("// this is a comment\nfn");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens, vec![RhlToken::KeywordFn, RhlToken::Eof]);
}

#[test]
fn tokenize_escape_sequences() {
    let mut lexer = Lexer::new(r#""\n\t\\""#);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            RhlToken::StringLiteral("\n\t\\".into()),
            RhlToken::Eof,
        ]
    );
}

#[test]
fn tokenize_struct_keyword() {
    let mut lexer = Lexer::new("struct Point { x: i32, y: i32 }");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(
        tokens,
        vec![
            RhlToken::KeywordStruct,
            RhlToken::Identifier("Point".into()),
            RhlToken::OpenBrace,
            RhlToken::Identifier("x".into()),
            RhlToken::Colon,
            RhlToken::TypeI32,
            RhlToken::Comma,
            RhlToken::Identifier("y".into()),
            RhlToken::Colon,
            RhlToken::TypeI32,
            RhlToken::CloseBrace,
            RhlToken::Eof,
        ]
    );
}
