// projects/products/unstable/simulation_compiler/backend/src/dsl/token.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Component,
    System,
    Event,
    Report,
    Ident(String),
    LBrace,
    RBrace,
    Colon,
    Comma,
    Eof,
}

pub struct Lexer<'src> {
    src: &'src str,
    pos: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self { src, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.src.len() {
            self.skip_whitespace();
            if self.pos >= self.src.len() {
                break;
            }
            let ch = self.current_char();
            match ch {
                '{' => { tokens.push(Token::LBrace); self.pos += 1; }
                '}' => { tokens.push(Token::RBrace); self.pos += 1; }
                ':' => { tokens.push(Token::Colon); self.pos += 1; }
                ',' => { tokens.push(Token::Comma); self.pos += 1; }
                _ if ch.is_alphabetic() || ch == '_' => {
                    let word = self.read_word();
                    let tok = match word.as_str() {
                        "component" => Token::Component,
                        "system" => Token::System,
                        "event" => Token::Event,
                        "report" => Token::Report,
                        other => Token::Ident(other.to_string()),
                    };
                    tokens.push(tok);
                }
                _ => { self.pos += 1; }
            }
        }
        tokens.push(Token::Eof);
        tokens
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.src.len() && self.src.as_bytes()[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn current_char(&self) -> char {
        self.src.as_bytes()[self.pos] as char
    }

    fn read_word(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.src.len() {
            let c = self.src.as_bytes()[self.pos] as char;
            if c.is_alphanumeric() || c == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        self.src[start..self.pos].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_component_block() {
        let mut lex = Lexer::new("component Foo { x: u32 }");
        let toks = lex.tokenize();
        assert_eq!(toks[0], Token::Component);
        assert_eq!(toks[1], Token::Ident("Foo".to_string()));
    }
}
