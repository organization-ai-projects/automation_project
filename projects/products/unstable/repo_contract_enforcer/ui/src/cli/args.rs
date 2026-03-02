// projects/products/unstable/repo_contract_enforcer/ui/src/cli/args.rs
use crate::cli::command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    pub json: bool,
    pub vscode: bool,
    pub command: command::Command,
}
