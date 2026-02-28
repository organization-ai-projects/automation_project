use crate::diagnostics::error::SpreadsheetError;
use crate::formula::ast::{BinOpKind, Expr};
use crate::formula::token::Token;
use crate::model::cell_id::CellId;

fn tokenize(input: &str) -> Result<Vec<Token>, SpreadsheetError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\r' | '\n' => { i += 1; }
            '=' => { tokens.push(Token::Equals); i += 1; }
            '+' => { tokens.push(Token::Plus); i += 1; }
            '-' => { tokens.push(Token::Minus); i += 1; }
            '*' => { tokens.push(Token::Star); i += 1; }
            '/' => { tokens.push(Token::Slash); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            ',' => { tokens.push(Token::Comma); i += 1; }
            '"' => {
                i += 1;
                let mut s = String::new();
                while i < chars.len() && chars[i] != '"' {
                    s.push(chars[i]);
                    i += 1;
                }
                if i >= chars.len() {
                    return Err(SpreadsheetError::ParseError("unclosed string literal".into()));
                }
                i += 1;
                tokens.push(Token::Text(s));
            }
            c if c.is_ascii_digit() || c == '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let num: f64 = num_str.parse().map_err(|_| {
                    SpreadsheetError::ParseError(format!("invalid number: {}", num_str))
                })?;
                tokens.push(Token::Number(num));
            }
            c if c.is_ascii_alphabetic() => {
                let start = i;
                // Collect letters
                while i < chars.len() && chars[i].is_ascii_alphabetic() {
                    i += 1;
                }
                let col_part: String = chars[start..i].iter().collect();
                // Check if followed by digits (cell ref)
                if i < chars.len() && chars[i].is_ascii_digit() {
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                    }
                    let cell_str: String = chars[start..i].iter().collect();
                    // Check for range
                    if i < chars.len() && chars[i] == ':' {
                        i += 1;
                        let range_start = i;
                        while i < chars.len() && chars[i].is_ascii_alphabetic() {
                            i += 1;
                        }
                        if i < chars.len() && chars[i].is_ascii_digit() {
                            while i < chars.len() && chars[i].is_ascii_digit() {
                                i += 1;
                            }
                        }
                        let to_str: String = chars[range_start..i].iter().collect();
                        tokens.push(Token::RangeRef(cell_str, to_str));
                    } else {
                        tokens.push(Token::CellRef(cell_str));
                    }
                } else {
                    // It's an identifier (function name)
                    tokens.push(Token::Ident(col_part));
                }
            }
            c => {
                return Err(SpreadsheetError::ParseError(format!("unexpected character: {}", c)));
            }
        }
    }
    Ok(tokens)
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn parse(formula: &str) -> Result<Expr, SpreadsheetError> {
        if !formula.starts_with('=') {
            return Err(SpreadsheetError::ParseError("formula must start with '='".into()));
        }
        let rest = &formula[1..];
        let tokens = tokenize(rest)?;
        let mut parser = Parser { tokens, pos: 0 };
        let expr = parser.parse_expr()?;
        if parser.pos < parser.tokens.len() {
            return Err(SpreadsheetError::ParseError(format!(
                "unexpected token at position {}", parser.pos
            )));
        }
        Ok(expr)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    fn parse_expr(&mut self) -> Result<Expr, SpreadsheetError> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expr, SpreadsheetError> {
        let mut lhs = self.parse_multiplicative()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.consume();
                    let rhs = self.parse_multiplicative()?;
                    lhs = Expr::BinOp { op: BinOpKind::Add, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                Some(Token::Minus) => {
                    self.consume();
                    let rhs = self.parse_multiplicative()?;
                    lhs = Expr::BinOp { op: BinOpKind::Sub, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, SpreadsheetError> {
        let mut lhs = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.consume();
                    let rhs = self.parse_unary()?;
                    lhs = Expr::BinOp { op: BinOpKind::Mul, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                Some(Token::Slash) => {
                    self.consume();
                    let rhs = self.parse_unary()?;
                    lhs = Expr::BinOp { op: BinOpKind::Div, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, SpreadsheetError> {
        if let Some(Token::Minus) = self.peek() {
            self.consume();
            let inner = self.parse_primary()?;
            return Ok(Expr::Neg(Box::new(inner)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, SpreadsheetError> {
        match self.peek().cloned() {
            Some(Token::Number(n)) => {
                self.consume();
                Ok(Expr::Number(n))
            }
            Some(Token::Text(s)) => {
                self.consume();
                Ok(Expr::Text(s))
            }
            Some(Token::CellRef(s)) => {
                self.consume();
                let id = CellId::from_a1(&s)?;
                Ok(Expr::CellRef(id))
            }
            Some(Token::RangeRef(from, to)) => {
                self.consume();
                let from_id = CellId::from_a1(&from)?;
                let to_id = CellId::from_a1(&to)?;
                Ok(Expr::RangeRef(from_id, to_id))
            }
            Some(Token::Ident(name)) => {
                self.consume();
                // Function call
                match self.peek() {
                    Some(Token::LParen) => {
                        self.consume();
                        let mut args = Vec::new();
                        if self.peek() != Some(&Token::RParen) {
                            args.push(self.parse_expr()?);
                            while self.peek() == Some(&Token::Comma) {
                                self.consume();
                                args.push(self.parse_expr()?);
                            }
                        }
                        match self.peek() {
                            Some(Token::RParen) => { self.consume(); }
                            _ => return Err(SpreadsheetError::ParseError("expected ')'".into())),
                        }
                        let upper = name.to_uppercase();
                        match upper.as_str() {
                            "SUM" | "MIN" | "MAX" => {}
                            _ => return Err(SpreadsheetError::ParseError(
                                format!("unknown function: {}", name)
                            )),
                        }
                        Ok(Expr::FunctionCall { name: upper, args })
                    }
                    _ => Err(SpreadsheetError::ParseError(format!("unexpected identifier: {}", name))),
                }
            }
            Some(Token::LParen) => {
                self.consume();
                let inner = self.parse_expr()?;
                match self.peek() {
                    Some(Token::RParen) => { self.consume(); }
                    _ => return Err(SpreadsheetError::ParseError("unclosed parenthesis".into())),
                }
                Ok(inner)
            }
            _ => Err(SpreadsheetError::ParseError(format!(
                "unexpected token at position {}", self.pos
            ))),
        }
    }
}

pub fn extract_deps(expr: &Expr) -> Vec<CellId> {
    let mut deps = Vec::new();
    collect_deps(expr, &mut deps);
    deps
}

fn collect_deps(expr: &Expr, deps: &mut Vec<CellId>) {
    match expr {
        Expr::CellRef(id) => deps.push(id.clone()),
        Expr::RangeRef(from, to) => {
            for row in from.row..=to.row {
                for col in from.col..=to.col {
                    deps.push(CellId::new(row, col));
                }
            }
        }
        Expr::BinOp { lhs, rhs, .. } => {
            collect_deps(lhs, deps);
            collect_deps(rhs, deps);
        }
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                collect_deps(arg, deps);
            }
        }
        Expr::Neg(inner) => collect_deps(inner, deps),
        _ => {}
    }
}
