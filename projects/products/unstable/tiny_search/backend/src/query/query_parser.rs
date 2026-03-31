use crate::query::query::Query;
use crate::tokenize::tokenizer::Tokenizer;

/// Parses a raw query string into a Query struct.
pub(crate) struct QueryParser;

impl QueryParser {
    pub(crate) fn parse(raw: &str) -> Query {
        let tokens = Tokenizer::tokenize(raw);
        let terms: Vec<String> = tokens.into_iter().map(|t| t.term).collect();
        Query { terms }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_determinism() {
        let a = QueryParser::parse("Hello World");
        let b = QueryParser::parse("Hello World");
        assert_eq!(a, b);
        assert_eq!(a.terms, vec!["hello", "world"]);
    }
}
