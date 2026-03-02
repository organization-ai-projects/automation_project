// projects/products/unstable/simulation_compiler/backend/src/dsl/parser.rs
use super::ast::{Ast, ComponentNode, EventNode, FieldDef, ReportNode, SystemNode};
use super::token::{Lexer, Token};
use crate::diagnostics::error::CompilerError;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(src: &str) -> Self {
        let mut lex = Lexer::new(src);
        let tokens = lex.tokenize();
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Ast, CompilerError> {
        let mut ast = Ast::default();
        while !self.at_eof() {
            match self.peek() {
                Token::Component => ast.components.push(self.parse_component()?),
                Token::System => ast.systems.push(self.parse_system()?),
                Token::Event => ast.events.push(self.parse_event()?),
                Token::Report => ast.reports.push(self.parse_report()?),
                Token::Eof => break,
                other => {
                    return Err(CompilerError::Parse(format!("unexpected token: {other:?}")));
                }
            }
        }
        Ok(ast)
    }

    fn parse_component(&mut self) -> Result<ComponentNode, CompilerError> {
        self.expect(Token::Component)?;
        let name = self.expect_ident()?;
        let fields = self.parse_field_block()?;
        Ok(ComponentNode { name, fields })
    }

    fn parse_system(&mut self) -> Result<SystemNode, CompilerError> {
        self.expect(Token::System)?;
        let name = self.expect_ident()?;
        let fields = self.parse_field_block()?;
        let reads = fields
            .iter()
            .filter(|f| f.ty == "read")
            .map(|f| f.name.clone())
            .collect();
        let writes = fields
            .iter()
            .filter(|f| f.ty == "write")
            .map(|f| f.name.clone())
            .collect();
        Ok(SystemNode {
            name,
            reads,
            writes,
        })
    }

    fn parse_event(&mut self) -> Result<EventNode, CompilerError> {
        self.expect(Token::Event)?;
        let name = self.expect_ident()?;
        let fields = self.parse_field_block()?;
        Ok(EventNode { name, fields })
    }

    fn parse_report(&mut self) -> Result<ReportNode, CompilerError> {
        self.expect(Token::Report)?;
        let name = self.expect_ident()?;
        let fields = self.parse_field_block()?;
        Ok(ReportNode { name, fields })
    }

    fn parse_field_block(&mut self) -> Result<Vec<FieldDef>, CompilerError> {
        self.expect(Token::LBrace)?;
        let mut fields = Vec::new();
        while self.peek() != &Token::RBrace && !self.at_eof() {
            let name = self.expect_ident()?;
            self.expect(Token::Colon)?;
            let ty = self.expect_ident()?;
            fields.push(FieldDef { name, ty });
            if self.peek() == &Token::Comma {
                self.advance();
            }
        }
        self.expect(Token::RBrace)?;
        Ok(fields)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn at_eof(&self) -> bool {
        self.peek() == &Token::Eof
    }

    fn expect(&mut self, expected: Token) -> Result<(), CompilerError> {
        let tok = self.advance().clone();
        if std::mem::discriminant(&tok) == std::mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(CompilerError::Parse(format!(
                "expected {expected:?}, found {tok:?}"
            )))
        }
    }

    fn expect_ident(&mut self) -> Result<String, CompilerError> {
        let tok = self.advance().clone();
        match tok {
            Token::Ident(s) => Ok(s),
            other => Err(CompilerError::Parse(format!(
                "expected identifier, found {other:?}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_component() {
        let mut p = Parser::new("component Sensor { value: u32 }");
        let ast = p.parse().unwrap();
        assert_eq!(ast.components.len(), 1);
        assert_eq!(ast.components[0].name, "Sensor");
        assert_eq!(ast.components[0].fields[0].name, "value");
    }

    #[test]
    fn parse_empty_ast() {
        let mut p = Parser::new("");
        let ast = p.parse().unwrap();
        assert!(ast.components.is_empty());
    }

    #[test]
    fn parse_event() {
        let mut p = Parser::new("event Collision { entity_a: u64, entity_b: u64 }");
        let ast = p.parse().unwrap();
        assert_eq!(ast.events.len(), 1);
        assert_eq!(ast.events[0].name, "Collision");
    }
}
