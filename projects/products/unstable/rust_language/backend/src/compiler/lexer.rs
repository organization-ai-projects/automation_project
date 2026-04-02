//! projects/products/unstable/rust_language/backend/src/compiler/lexer.rs
use crate::{diagnostics::Error, model::RhlToken};

pub(crate) struct Lexer {
    pub(crate) input: Vec<char>,
    pub(crate) pos: usize,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl Lexer {
    pub(crate) fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub(crate) fn tokenize(&mut self) -> Result<Vec<RhlToken>, Error> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            self.skip_whitespace();
            if self.pos >= self.input.len() {
                break;
            }
            let token = self.next_token()?;
            tokens.push(token);
        }
        tokens.push(RhlToken::Eof);
        Ok(tokens)
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.pos).copied();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '/' && self.input.get(self.pos + 1).copied() == Some('/') {
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Result<RhlToken, Error> {
        let ch = self.peek().ok_or_else(|| Error::Lexer {
            line: self.line,
            col: self.col,
            message: "unexpected end of input".into(),
        })?;

        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.read_identifier_or_keyword();
        }
        if ch.is_ascii_digit() {
            return self.read_number();
        }
        if ch == '"' {
            return self.read_string();
        }

        self.read_symbol()
    }

    fn read_identifier_or_keyword(&mut self) -> Result<RhlToken, Error> {
        let mut word = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                word.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let token = match word.as_str() {
            "fn" => RhlToken::KeywordFn,
            "let" => RhlToken::KeywordLet,
            "mut" => RhlToken::KeywordMut,
            "return" => RhlToken::KeywordReturn,
            "if" => RhlToken::KeywordIf,
            "else" => RhlToken::KeywordElse,
            "struct" => RhlToken::KeywordStruct,
            "impl" => RhlToken::KeywordImpl,
            "for" => RhlToken::KeywordFor,
            "while" => RhlToken::KeywordWhile,
            "pub" => RhlToken::KeywordPub,
            "use" => RhlToken::KeywordUse,
            "true" => RhlToken::BoolLiteral(true),
            "false" => RhlToken::BoolLiteral(false),
            "i32" => RhlToken::TypeI32,
            "i64" => RhlToken::TypeI64,
            "f64" => RhlToken::TypeF64,
            "bool" => RhlToken::TypeBool,
            "String" => RhlToken::TypeString,
            _ => RhlToken::Identifier(word),
        };
        Ok(token)
    }

    fn read_number(&mut self) -> Result<RhlToken, Error> {
        let mut num_str = String::new();
        let mut has_dot = false;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if has_dot {
            let val: f64 = num_str.parse().map_err(|_| Error::Lexer {
                line: self.line,
                col: self.col,
                message: format!("invalid float literal: {num_str}"),
            })?;
            Ok(RhlToken::FloatLiteral(val))
        } else {
            let val: i64 = num_str.parse().map_err(|_| Error::Lexer {
                line: self.line,
                col: self.col,
                message: format!("invalid integer literal: {num_str}"),
            })?;
            Ok(RhlToken::IntegerLiteral(val))
        }
    }

    fn read_string(&mut self) -> Result<RhlToken, Error> {
        self.advance(); // consume opening "
        let mut s = String::new();
        loop {
            match self.advance() {
                Some('"') => return Ok(RhlToken::StringLiteral(s)),
                Some('\\') => match self.advance() {
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('\\') => s.push('\\'),
                    Some('"') => s.push('"'),
                    _ => {
                        return Err(Error::Lexer {
                            line: self.line,
                            col: self.col,
                            message: "invalid escape sequence".into(),
                        });
                    }
                },
                Some(ch) => s.push(ch),
                None => {
                    return Err(Error::Lexer {
                        line: self.line,
                        col: self.col,
                        message: "unterminated string literal".into(),
                    });
                }
            }
        }
    }

    fn read_symbol(&mut self) -> Result<RhlToken, Error> {
        let ch = self.advance().unwrap();
        let next = self.peek();
        match ch {
            '-' if next == Some('>') => {
                self.advance();
                Ok(RhlToken::Arrow)
            }
            '=' if next == Some('>') => {
                self.advance();
                Ok(RhlToken::FatArrow)
            }
            '=' if next == Some('=') => {
                self.advance();
                Ok(RhlToken::DoubleEquals)
            }
            '!' if next == Some('=') => {
                self.advance();
                Ok(RhlToken::NotEquals)
            }
            '<' if next == Some('=') => {
                self.advance();
                Ok(RhlToken::LessEqual)
            }
            '>' if next == Some('=') => {
                self.advance();
                Ok(RhlToken::GreaterEqual)
            }
            ':' => Ok(RhlToken::Colon),
            ';' => Ok(RhlToken::Semicolon),
            ',' => Ok(RhlToken::Comma),
            '.' => Ok(RhlToken::Dot),
            '(' => Ok(RhlToken::OpenParen),
            ')' => Ok(RhlToken::CloseParen),
            '{' => Ok(RhlToken::OpenBrace),
            '}' => Ok(RhlToken::CloseBrace),
            '[' => Ok(RhlToken::OpenBracket),
            ']' => Ok(RhlToken::CloseBracket),
            '=' => Ok(RhlToken::Equals),
            '+' => Ok(RhlToken::Plus),
            '-' => Ok(RhlToken::Minus),
            '*' => Ok(RhlToken::Star),
            '/' => Ok(RhlToken::Slash),
            '<' => Ok(RhlToken::LessThan),
            '>' => Ok(RhlToken::GreaterThan),
            '&' => Ok(RhlToken::Ampersand),
            '|' => Ok(RhlToken::Pipe),
            '!' => Ok(RhlToken::Bang),
            _ => Err(Error::Lexer {
                line: self.line,
                col: self.col,
                message: format!("unexpected character: {ch}"),
            }),
        }
    }
}
