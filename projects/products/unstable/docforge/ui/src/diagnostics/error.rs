#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Transport(String),
}
