/// A single token extracted from text.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Token {
    pub(crate) term: String,
    pub(crate) position: usize,
}

impl Token {
    pub(crate) fn new(term: String, position: usize) -> Self {
        Self { term, position }
    }
}
