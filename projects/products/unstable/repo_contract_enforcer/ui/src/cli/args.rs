#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub json: bool,
    pub command: super::command::Command,
}
