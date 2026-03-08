#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Ok,
    Error(String),
}
