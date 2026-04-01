use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum FuzzResult {
    Pass,
    Fail(String),
}

impl fmt::Display for FuzzResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FuzzResult::Pass => write!(f, "PASS"),
            FuzzResult::Fail(msg) => write!(f, "FAIL: {msg}"),
        }
    }
}
