// projects/libraries/command_runner/src/command_info.rs
#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub program: String,
    pub args: Vec<String>,
}
