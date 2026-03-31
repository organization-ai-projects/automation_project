#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    State,
    Transition,
    Invariant,
    Arrow,
    On,
    Ident(String),
    LBrace,
    RBrace,
    Colon,
    Comma,
    StringLit(String),
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok == Token::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Token::Eof;
        }

        let ch = self.input[self.pos];

        match ch {
            '{' => {
                self.pos += 1;
                Token::LBrace
            }
            '}' => {
                self.pos += 1;
                Token::RBrace
            }
            ':' => {
                self.pos += 1;
                Token::Colon
            }
            ',' => {
                self.pos += 1;
                Token::Comma
            }
            '-' if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '>' => {
                self.pos += 2;
                Token::Arrow
            }
            '"' => self.read_string(),
            _ if ch.is_alphabetic() || ch == '_' => self.read_ident(),
            _ => {
                self.pos += 1;
                self.next_token()
            }
        }
    }

    fn read_ident(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_')
        {
            self.pos += 1;
        }
        let word: String = self.input[start..self.pos].iter().collect();
        match word.as_str() {
            "state" => Token::State,
            "transition" => Token::Transition,
            "invariant" => Token::Invariant,
            "on" => Token::On,
            _ => Token::Ident(word),
        }
    }

    fn read_string(&mut self) -> Token {
        self.pos += 1; // skip opening quote
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        if self.pos < self.input.len() {
            self.pos += 1; // skip closing quote
        }
        Token::StringLit(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_state_block() {
        let mut lexer = Lexer::new("state Idle {}");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::State,
                Token::Ident("Idle".to_string()),
                Token::LBrace,
                Token::RBrace,
                Token::Eof,
            ]
        );
    }
}
