use crate::diagnostics::error::CompilerError;
use crate::dsl::ast::{FieldDef, InvariantNode, SpecAst, StateNode, TransitionNode};
use crate::dsl::token::{Lexer, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<SpecAst, CompilerError> {
        let mut ast = SpecAst::default();

        loop {
            match self.peek() {
                Token::Eof => break,
                Token::State => {
                    let node = self.parse_state()?;
                    ast.states.push(node);
                }
                Token::Transition => {
                    let node = self.parse_transition()?;
                    ast.transitions.push(node);
                }
                Token::Invariant => {
                    let node = self.parse_invariant()?;
                    ast.invariants.push(node);
                }
                Token::Initial => {
                    let name = self.parse_initial()?;
                    if ast.initial_state.is_some() {
                        return Err(CompilerError::Parse(
                            "duplicate initial state declaration".to_string(),
                        ));
                    }
                    ast.initial_state = Some(name);
                }
                Token::Error(ch) => {
                    return Err(CompilerError::Parse(format!(
                        "unexpected character: '{}'",
                        ch
                    )));
                }
                other => {
                    return Err(CompilerError::Parse(format!(
                        "unexpected token: {:?}",
                        other
                    )));
                }
            }
        }

        Ok(ast)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    fn expect_ident(&mut self) -> Result<String, CompilerError> {
        match self.advance() {
            Token::Ident(s) => Ok(s),
            other => Err(CompilerError::Parse(format!(
                "expected identifier, got {:?}",
                other
            ))),
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), CompilerError> {
        let tok = self.advance();
        if &tok == expected {
            Ok(())
        } else {
            Err(CompilerError::Parse(format!(
                "expected {:?}, got {:?}",
                expected, tok
            )))
        }
    }

    fn parse_state(&mut self) -> Result<StateNode, CompilerError> {
        self.expect(&Token::State)?;
        let name = self.expect_ident()?;
        self.expect(&Token::LBrace)?;

        let mut fields = Vec::new();
        while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
            let field_name = self.expect_ident()?;
            self.expect(&Token::Colon)?;
            let ty = self.expect_ident()?;
            fields.push(FieldDef {
                name: field_name,
                ty,
            });
            if *self.peek() == Token::Comma {
                self.advance();
            }
        }
        self.expect(&Token::RBrace)?;

        Ok(StateNode { name, fields })
    }

    fn parse_transition(&mut self) -> Result<TransitionNode, CompilerError> {
        self.expect(&Token::Transition)?;
        let from = self.expect_ident()?;
        self.expect(&Token::Arrow)?;
        let to = self.expect_ident()?;
        self.expect(&Token::On)?;
        let event = self.expect_ident()?;
        self.expect(&Token::LBrace)?;

        let mut guard_fields = Vec::new();
        while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
            let field_name = self.expect_ident()?;
            self.expect(&Token::Colon)?;
            let ty = self.expect_ident()?;
            guard_fields.push(FieldDef {
                name: field_name,
                ty,
            });
            if *self.peek() == Token::Comma {
                self.advance();
            }
        }
        self.expect(&Token::RBrace)?;

        Ok(TransitionNode {
            from,
            to,
            event,
            guard_fields,
        })
    }

    fn parse_invariant(&mut self) -> Result<InvariantNode, CompilerError> {
        self.expect(&Token::Invariant)?;
        let name = self.expect_ident()?;
        let description = match self.advance() {
            Token::StringLit(s) => s,
            other => {
                return Err(CompilerError::Parse(format!(
                    "expected string literal for invariant description, got {:?}",
                    other
                )));
            }
        };
        Ok(InvariantNode { name, description })
    }

    fn parse_initial(&mut self) -> Result<String, CompilerError> {
        self.expect(&Token::Initial)?;
        self.expect_ident()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_state() {
        let mut parser = Parser::new("state Idle { tick: u64 }");
        let ast = parser.parse().unwrap();
        assert_eq!(ast.states.len(), 1);
        assert_eq!(ast.states[0].name, "Idle");
        assert_eq!(ast.states[0].fields.len(), 1);
        assert_eq!(ast.states[0].fields[0].name, "tick");
        assert_eq!(ast.states[0].fields[0].ty, "u64");
    }

    #[test]
    fn parse_empty_ast() {
        let mut parser = Parser::new("");
        let ast = parser.parse().unwrap();
        assert!(ast.states.is_empty());
        assert!(ast.transitions.is_empty());
        assert!(ast.invariants.is_empty());
    }

    #[test]
    fn parse_transition() {
        let mut parser =
            Parser::new("state Idle {}\nstate Running {}\ntransition Idle -> Running on start {}");
        let ast = parser.parse().unwrap();
        assert_eq!(ast.transitions.len(), 1);
        assert_eq!(ast.transitions[0].from, "Idle");
        assert_eq!(ast.transitions[0].to, "Running");
        assert_eq!(ast.transitions[0].event, "start");
    }

    #[test]
    fn rejects_unrecognized_character() {
        let mut parser = Parser::new("@state Idle {}");
        let result = parser.parse();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unexpected character"));
    }

    #[test]
    fn parse_initial_state() {
        let mut parser = Parser::new("state Idle {}\nstate Running {}\ninitial Idle");
        let ast = parser.parse().unwrap();
        assert_eq!(ast.initial_state, Some("Idle".to_string()));
    }

    #[test]
    fn rejects_duplicate_initial() {
        let mut parser =
            Parser::new("state A {}\nstate B {}\ninitial A\ninitial B");
        let result = parser.parse();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("duplicate initial state"));
    }
}
