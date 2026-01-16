use std::fmt;

/// A key in an AST object.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AstKey {
    /// An identifier key (e.g., `foo` in `{ foo: 1 }`)
    Ident(String),
    /// A string key (e.g., `"foo-bar"` in `{ "foo-bar": 1 }`)
    String(String),
}

impl AstKey {
    /// Returns the key as a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            AstKey::Ident(s) | AstKey::String(s) => s,
        }
    }

    /// Returns true if this is an identifier key.
    pub fn is_ident(&self) -> bool {
        matches!(self, AstKey::Ident(_))
    }

    /// Returns true if this is a string key.
    pub fn is_string(&self) -> bool {
        matches!(self, AstKey::String(_))
    }
}

impl fmt::Display for AstKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for AstKey {
    fn from(s: &str) -> Self {
        AstKey::String(s.to_string())
    }
}

impl From<String> for AstKey {
    fn from(s: String) -> Self {
        AstKey::String(s)
    }
}
