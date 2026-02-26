// projects/products/unstable/autonomy_orchestrator_ai/src/cli_command/cli.rs
use clap::Parser;

use crate::{Commands, run_args::RunArgs};

#[derive(Parser, Debug)]
#[command(name = "autonomy_orchestrator_ai")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[command(flatten)]
    pub run: RunArgs,
}
