use crate::diagnostics::error::Error;
use crate::model::rhl_ast::RhlAst;
use crate::model::rhl_token::RhlToken;

pub struct Parser {
    tokens: Vec<RhlToken>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<RhlToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<RhlAst, Error> {
        let mut items = Vec::new();
        while !self.is_at_end() {
            let item = self.parse_top_level()?;
            items.push(item);
        }
        Ok(RhlAst::Program(items))
    }

    fn peek(&self) -> &RhlToken {
        self.tokens.get(self.pos).unwrap_or(&RhlToken::Eof)
    }

    fn advance(&mut self) -> &RhlToken {
        let tok = self.tokens.get(self.pos).unwrap_or(&RhlToken::Eof);
        self.pos += 1;
        tok
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), RhlToken::Eof)
    }

    fn expect(&mut self, expected: &RhlToken) -> Result<(), Error> {
        let tok = self.peek().clone();
        if std::mem::discriminant(&tok) == std::mem::discriminant(expected) {
            self.advance();
            Ok(())
        } else {
            Err(Error::Parser {
                line: 0,
                message: format!("expected {expected:?}, found {tok:?}"),
            })
        }
    }

    fn parse_top_level(&mut self) -> Result<RhlAst, Error> {
        match self.peek() {
            RhlToken::KeywordFn => self.parse_fn_decl(),
            RhlToken::KeywordStruct => self.parse_struct_decl(),
            RhlToken::KeywordPub => {
                self.advance();
                match self.peek() {
                    RhlToken::KeywordFn => self.parse_fn_decl(),
                    RhlToken::KeywordStruct => self.parse_struct_decl(),
                    _ => Err(Error::Parser {
                        line: 0,
                        message: "expected fn or struct after pub".into(),
                    }),
                }
            }
            _ => Err(Error::Parser {
                line: 0,
                message: format!("unexpected top-level token: {:?}", self.peek()),
            }),
        }
    }

    fn parse_fn_decl(&mut self) -> Result<RhlAst, Error> {
        self.expect(&RhlToken::KeywordFn)?;
        let name = self.parse_identifier()?;
        self.expect(&RhlToken::OpenParen)?;
        let params = self.parse_param_list()?;
        self.expect(&RhlToken::CloseParen)?;

        let return_type = if matches!(self.peek(), RhlToken::Arrow) {
            self.advance();
            Some(self.parse_type_name()?)
        } else {
            None
        };

        self.expect(&RhlToken::OpenBrace)?;
        let body = self.parse_block_body()?;
        self.expect(&RhlToken::CloseBrace)?;

        Ok(RhlAst::FnDecl {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_struct_decl(&mut self) -> Result<RhlAst, Error> {
        self.expect(&RhlToken::KeywordStruct)?;
        let name = self.parse_identifier()?;
        self.expect(&RhlToken::OpenBrace)?;
        let mut fields = Vec::new();
        while !matches!(self.peek(), RhlToken::CloseBrace | RhlToken::Eof) {
            let field_name = self.parse_identifier()?;
            self.expect(&RhlToken::Colon)?;
            let field_type = self.parse_type_name()?;
            fields.push((field_name, field_type));
            if matches!(self.peek(), RhlToken::Comma) {
                self.advance();
            }
        }
        self.expect(&RhlToken::CloseBrace)?;
        Ok(RhlAst::StructDecl { name, fields })
    }

    fn parse_param_list(&mut self) -> Result<Vec<(String, String)>, Error> {
        let mut params = Vec::new();
        while !matches!(self.peek(), RhlToken::CloseParen | RhlToken::Eof) {
            let name = self.parse_identifier()?;
            self.expect(&RhlToken::Colon)?;
            let type_name = self.parse_type_name()?;
            params.push((name, type_name));
            if matches!(self.peek(), RhlToken::Comma) {
                self.advance();
            }
        }
        Ok(params)
    }

    fn parse_type_name(&mut self) -> Result<String, Error> {
        let tok = self.peek().clone();
        match tok {
            RhlToken::TypeI32 => {
                self.advance();
                Ok("i32".into())
            }
            RhlToken::TypeI64 => {
                self.advance();
                Ok("i64".into())
            }
            RhlToken::TypeF64 => {
                self.advance();
                Ok("f64".into())
            }
            RhlToken::TypeBool => {
                self.advance();
                Ok("bool".into())
            }
            RhlToken::TypeString => {
                self.advance();
                Ok("String".into())
            }
            RhlToken::Identifier(name) => {
                self.advance();
                Ok(name)
            }
            _ => Err(Error::Parser {
                line: 0,
                message: format!("expected type name, found {tok:?}"),
            }),
        }
    }

    fn parse_block_body(&mut self) -> Result<Vec<RhlAst>, Error> {
        let mut stmts = Vec::new();
        while !matches!(self.peek(), RhlToken::CloseBrace | RhlToken::Eof) {
            let stmt = self.parse_statement()?;
            stmts.push(stmt);
        }
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<RhlAst, Error> {
        match self.peek() {
            RhlToken::KeywordLet => self.parse_let_binding(),
            RhlToken::KeywordReturn => self.parse_return(),
            RhlToken::KeywordIf => self.parse_if_expr(),
            RhlToken::KeywordWhile => self.parse_while_loop(),
            _ => {
                let expr = self.parse_expression()?;
                if matches!(self.peek(), RhlToken::Semicolon) {
                    self.advance();
                }
                Ok(expr)
            }
        }
    }

    fn parse_let_binding(&mut self) -> Result<RhlAst, Error> {
        self.expect(&RhlToken::KeywordLet)?;
        let mutable = if matches!(self.peek(), RhlToken::KeywordMut) {
            self.advance();
            true
        } else {
            false
        };
        let name = self.parse_identifier()?;
        let type_annotation = if matches!(self.peek(), RhlToken::Colon) {
            self.advance();
            Some(self.parse_type_name()?)
        } else {
            None
        };
        self.expect(&RhlToken::Equals)?;
        let value = self.parse_expression()?;
        self.expect(&RhlToken::Semicolon)?;
        Ok(RhlAst::LetBinding {
            name,
            mutable,
            type_annotation,
            value: Box::new(value),
        })
    }

    fn parse_return(&mut self) -> Result<RhlAst, Error> {
        self.expect(&RhlToken::KeywordReturn)?;
        let value = self.parse_expression()?;
        self.expect(&RhlToken::Semicolon)?;
        Ok(RhlAst::Return(Box::new(value)))
    }

    fn parse_if_expr(&mut self) -> Result<RhlAst, Error> {
        self.expect(&RhlToken::KeywordIf)?;
        let condition = self.parse_expression()?;
        self.expect(&RhlToken::OpenBrace)?;
        let then_body = self.parse_block_body()?;
        self.expect(&RhlToken::CloseBrace)?;
        let else_body = if matches!(self.peek(), RhlToken::KeywordElse) {
            self.advance();
            self.expect(&RhlToken::OpenBrace)?;
            let body = self.parse_block_body()?;
            self.expect(&RhlToken::CloseBrace)?;
            Some(body)
        } else {
            None
        };
        Ok(RhlAst::IfExpr {
            condition: Box::new(condition),
            then_body,
            else_body,
        })
    }

    fn parse_while_loop(&mut self) -> Result<RhlAst, Error> {
        self.expect(&RhlToken::KeywordWhile)?;
        let condition = self.parse_expression()?;
        self.expect(&RhlToken::OpenBrace)?;
        let body = self.parse_block_body()?;
        self.expect(&RhlToken::CloseBrace)?;
        Ok(RhlAst::WhileLoop {
            condition: Box::new(condition),
            body,
        })
    }

    fn parse_expression(&mut self) -> Result<RhlAst, Error> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<RhlAst, Error> {
        let mut left = self.parse_additive()?;
        while matches!(
            self.peek(),
            RhlToken::DoubleEquals
                | RhlToken::NotEquals
                | RhlToken::LessThan
                | RhlToken::GreaterThan
                | RhlToken::LessEqual
                | RhlToken::GreaterEqual
        ) {
            let op = self.advance().clone();
            let op_str = match op {
                RhlToken::DoubleEquals => "==",
                RhlToken::NotEquals => "!=",
                RhlToken::LessThan => "<",
                RhlToken::GreaterThan => ">",
                RhlToken::LessEqual => "<=",
                RhlToken::GreaterEqual => ">=",
                _ => unreachable!(),
            };
            let right = self.parse_additive()?;
            left = RhlAst::BinaryOp {
                left: Box::new(left),
                op: op_str.into(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<RhlAst, Error> {
        let mut left = self.parse_multiplicative()?;
        while matches!(self.peek(), RhlToken::Plus | RhlToken::Minus) {
            let op = if matches!(self.peek(), RhlToken::Plus) {
                self.advance();
                "+"
            } else {
                self.advance();
                "-"
            };
            let right = self.parse_multiplicative()?;
            left = RhlAst::BinaryOp {
                left: Box::new(left),
                op: op.into(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<RhlAst, Error> {
        let mut left = self.parse_primary()?;
        while matches!(self.peek(), RhlToken::Star | RhlToken::Slash) {
            let op = if matches!(self.peek(), RhlToken::Star) {
                self.advance();
                "*"
            } else {
                self.advance();
                "/"
            };
            let right = self.parse_primary()?;
            left = RhlAst::BinaryOp {
                left: Box::new(left),
                op: op.into(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<RhlAst, Error> {
        let tok = self.peek().clone();
        match tok {
            RhlToken::IntegerLiteral(v) => {
                self.advance();
                Ok(RhlAst::IntLiteral(v))
            }
            RhlToken::FloatLiteral(v) => {
                self.advance();
                Ok(RhlAst::FloatLiteral(v))
            }
            RhlToken::StringLiteral(ref s) => {
                let val = s.clone();
                self.advance();
                Ok(RhlAst::StringLiteral(val))
            }
            RhlToken::BoolLiteral(v) => {
                self.advance();
                Ok(RhlAst::BoolLiteral(v))
            }
            RhlToken::Identifier(ref name) => {
                let name = name.clone();
                self.advance();
                if matches!(self.peek(), RhlToken::OpenParen) {
                    self.advance();
                    let args = self.parse_arg_list()?;
                    self.expect(&RhlToken::CloseParen)?;
                    Ok(RhlAst::Call {
                        callee: name,
                        args,
                    })
                } else if matches!(self.peek(), RhlToken::Dot) {
                    self.advance();
                    let field = self.parse_identifier()?;
                    Ok(RhlAst::FieldAccess {
                        object: Box::new(RhlAst::Identifier(name)),
                        field,
                    })
                } else if matches!(self.peek(), RhlToken::Equals)
                    && !matches!(self.peek(), RhlToken::DoubleEquals)
                {
                    // Check if it's actually `=` (assignment) and not `==`
                    if let Some(next) = self.tokens.get(self.pos) {
                        if matches!(next, RhlToken::Equals) {
                            // Check the token after `=`
                            if let Some(after) = self.tokens.get(self.pos + 1) {
                                if matches!(after, RhlToken::Equals) {
                                    // It's `==` handled by comparison
                                    return Ok(RhlAst::Identifier(name));
                                }
                            }
                            self.advance();
                            let value = self.parse_expression()?;
                            if matches!(self.peek(), RhlToken::Semicolon) {
                                self.advance();
                            }
                            return Ok(RhlAst::Assignment {
                                target: name,
                                value: Box::new(value),
                            });
                        }
                    }
                    Ok(RhlAst::Identifier(name))
                } else {
                    Ok(RhlAst::Identifier(name))
                }
            }
            RhlToken::OpenParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&RhlToken::CloseParen)?;
                Ok(expr)
            }
            _ => Err(Error::Parser {
                line: 0,
                message: format!("unexpected token in expression: {tok:?}"),
            }),
        }
    }

    fn parse_arg_list(&mut self) -> Result<Vec<RhlAst>, Error> {
        let mut args = Vec::new();
        while !matches!(self.peek(), RhlToken::CloseParen | RhlToken::Eof) {
            let arg = self.parse_expression()?;
            args.push(arg);
            if matches!(self.peek(), RhlToken::Comma) {
                self.advance();
            }
        }
        Ok(args)
    }

    fn parse_identifier(&mut self) -> Result<String, Error> {
        let tok = self.peek().clone();
        if let RhlToken::Identifier(name) = tok {
            self.advance();
            Ok(name)
        } else {
            Err(Error::Parser {
                line: 0,
                message: format!("expected identifier, found {tok:?}"),
            })
        }
    }
}
