use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;
use crate::compiler::transpiler::Transpiler;

#[test]
fn transpile_simple_function() {
    let source = "fn main() { let x = 42; }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let rust_code = Transpiler::transpile(&ast).unwrap();
    assert!(rust_code.contains("fn main()"));
    assert!(rust_code.contains("let x = 42;"));
}

#[test]
fn transpile_function_with_return_type() {
    let source = "fn add(a: i32, b: i32) -> i32 { return a + b; }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let rust_code = Transpiler::transpile(&ast).unwrap();
    assert!(rust_code.contains("fn add(a: i32, b: i32) -> i32"));
    assert!(rust_code.contains("return a + b;"));
}

#[test]
fn transpile_struct_declaration() {
    let source = "struct Point { x: f64, y: f64 }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let rust_code = Transpiler::transpile(&ast).unwrap();
    assert!(rust_code.contains("struct Point"));
    assert!(rust_code.contains("x: f64,"));
    assert!(rust_code.contains("y: f64,"));
}

#[test]
fn transpile_if_else() {
    let source = "fn check(x: i32) -> i32 { if x > 0 { return x; } else { return 0; } }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let rust_code = Transpiler::transpile(&ast).unwrap();
    assert!(rust_code.contains("if x > 0"));
    assert!(rust_code.contains("else"));
    assert!(rust_code.contains("return x;"));
    assert!(rust_code.contains("return 0;"));
}

#[test]
fn transpile_while_loop() {
    let source = "fn count() { let mut i = 0; while i < 10 { i = i + 1; } }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let rust_code = Transpiler::transpile(&ast).unwrap();
    assert!(rust_code.contains("while i < 10"));
    assert!(rust_code.contains("let mut i = 0;"));
}

#[test]
fn transpile_string_literal() {
    let source = r#"fn greet() { let msg = "hello"; }"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let rust_code = Transpiler::transpile(&ast).unwrap();
    assert!(rust_code.contains(r#""hello""#));
}

#[test]
fn transpile_mutable_binding() {
    let source = "fn main() { let mut count: i32 = 0; }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let rust_code = Transpiler::transpile(&ast).unwrap();
    assert!(rust_code.contains("let mut count: i32 = 0;"));
}

#[test]
fn transpile_bool_literal() {
    let source = "fn main() { let flag = true; }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let rust_code = Transpiler::transpile(&ast).unwrap();
    assert!(rust_code.contains("let flag = true;"));
}

#[test]
fn full_pipeline_roundtrip() {
    let source = r#"
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let result = add(1, 2);
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let rust_code = Transpiler::transpile(&ast).unwrap();

    assert!(rust_code.contains("fn add(a: i32, b: i32) -> i32"));
    assert!(rust_code.contains("return a + b;"));
    assert!(rust_code.contains("fn main()"));
    assert!(rust_code.contains("let result = add(1, 2);"));
}
