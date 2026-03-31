use crate::tokenize::token::Token;

/// Deterministic tokenizer: lowercases, splits on non-alphanumeric, filters empty tokens.
pub(crate) struct Tokenizer;

impl Tokenizer {
    pub(crate) fn tokenize(text: &str) -> Vec<Token> {
        let lower = text.to_lowercase();
        let mut tokens = Vec::new();
        let mut position: usize = 0;
        for word in lower.split(|c: char| !c.is_alphanumeric()) {
            if word.is_empty() {
                continue;
            }
            tokens.push(Token::new(word.to_string(), position));
            position += 1;
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_determinism() {
        let text = "Hello, World! This is a TEST.";
        let a = Tokenizer::tokenize(text);
        let b = Tokenizer::tokenize(text);
        assert_eq!(a, b);
        let terms: Vec<&str> = a.iter().map(|t| t.term.as_str()).collect();
        assert_eq!(terms, vec!["hello", "world", "this", "is", "a", "test"]);
    }

    #[test]
    fn tokenize_empty() {
        let tokens = Tokenizer::tokenize("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn tokenize_positions() {
        let tokens = Tokenizer::tokenize("one two three");
        assert_eq!(tokens[0].position, 0);
        assert_eq!(tokens[1].position, 1);
        assert_eq!(tokens[2].position, 2);
    }

    #[test]
    fn tokenize_special_chars() {
        let tokens = Tokenizer::tokenize("a--b__c..d");
        let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_str()).collect();
        assert_eq!(terms, vec!["a", "b", "c", "d"]);
    }
}
