#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub lexeme: String,
}

impl Token {
    pub fn new(lexeme: impl Into<String>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}
