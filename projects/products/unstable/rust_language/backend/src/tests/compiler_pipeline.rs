use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;
use crate::model::rhl_ast::RhlAst;

#[test]
fn parse_simple_function() {
    let source = "fn main() { let x = 42; }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    match ast {
        RhlAst::Program(items) => {
            assert_eq!(items.len(), 1);
            match &items[0] {
                RhlAst::FnDecl {
                    name,
                    params,
                    return_type,
                    body,
                } => {
                    assert_eq!(name, "main");
                    assert!(params.is_empty());
                    assert!(return_type.is_none());
                    assert_eq!(body.len(), 1);
                }
                other => panic!("expected FnDecl, got {other:?}"),
            }
        }
        other => panic!("expected Program, got {other:?}"),
    }
}

#[test]
fn parse_function_with_params_and_return() {
    let source = "fn add(a: i32, b: i32) -> i32 { return a + b; }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    match ast {
        RhlAst::Program(items) => {
            assert_eq!(items.len(), 1);
            match &items[0] {
                RhlAst::FnDecl {
                    name,
                    params,
                    return_type,
                    body,
                } => {
                    assert_eq!(name, "add");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params[0], ("a".into(), "i32".into()));
                    assert_eq!(params[1], ("b".into(), "i32".into()));
                    assert_eq!(return_type.as_deref(), Some("i32"));
                    assert_eq!(body.len(), 1);
                }
                other => panic!("expected FnDecl, got {other:?}"),
            }
        }
        other => panic!("expected Program, got {other:?}"),
    }
}

#[test]
fn parse_struct_declaration() {
    let source = "struct Point { x: f64, y: f64 }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    match ast {
        RhlAst::Program(items) => {
            assert_eq!(items.len(), 1);
            match &items[0] {
                RhlAst::StructDecl { name, fields } => {
                    assert_eq!(name, "Point");
                    assert_eq!(fields.len(), 2);
                    assert_eq!(fields[0], ("x".into(), "f64".into()));
                    assert_eq!(fields[1], ("y".into(), "f64".into()));
                }
                other => panic!("expected StructDecl, got {other:?}"),
            }
        }
        other => panic!("expected Program, got {other:?}"),
    }
}

#[test]
fn parse_if_else() {
    let source = "fn check(x: i32) { if x > 0 { return x; } else { return 0; } }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    match ast {
        RhlAst::Program(items) => {
            assert_eq!(items.len(), 1);
            match &items[0] {
                RhlAst::FnDecl { body, .. } => {
                    assert_eq!(body.len(), 1);
                    match &body[0] {
                        RhlAst::IfExpr {
                            else_body: Some(else_stmts),
                            ..
                        } => {
                            assert!(!else_stmts.is_empty());
                        }
                        other => panic!("expected IfExpr with else, got {other:?}"),
                    }
                }
                other => panic!("expected FnDecl, got {other:?}"),
            }
        }
        other => panic!("expected Program, got {other:?}"),
    }
}

#[test]
fn parse_while_loop() {
    let source = "fn count() { let mut i = 0; while i < 10 { i = i + 1; } }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    match ast {
        RhlAst::Program(items) => {
            assert_eq!(items.len(), 1);
            match &items[0] {
                RhlAst::FnDecl { body, .. } => {
                    assert!(body.len() >= 2);
                }
                other => panic!("expected FnDecl, got {other:?}"),
            }
        }
        other => panic!("expected Program, got {other:?}"),
    }
}

#[test]
fn parse_function_call() {
    let source = "fn main() { print(42); }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    match ast {
        RhlAst::Program(items) => {
            assert_eq!(items.len(), 1);
            match &items[0] {
                RhlAst::FnDecl { body, .. } => {
                    assert_eq!(body.len(), 1);
                    match &body[0] {
                        RhlAst::Call { callee, args } => {
                            assert_eq!(callee, "print");
                            assert_eq!(args.len(), 1);
                        }
                        other => panic!("expected Call, got {other:?}"),
                    }
                }
                other => panic!("expected FnDecl, got {other:?}"),
            }
        }
        other => panic!("expected Program, got {other:?}"),
    }
}

#[test]
fn parse_multiple_functions() {
    let source = "fn foo() {} fn bar() {}";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    match ast {
        RhlAst::Program(items) => {
            assert_eq!(items.len(), 2);
        }
        other => panic!("expected Program, got {other:?}"),
    }
}
