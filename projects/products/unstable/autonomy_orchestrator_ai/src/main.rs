// projects/products/unstable/autonomy_orchestrator_ai/src/main.rs
mod binary_runner;
mod checkpoint_store;
mod cli_command;
mod commands;
mod configs;
mod domain;
mod fixture;
mod linked_stack;
mod orchestrator;
mod output_writer;
mod pending_validation_invocation;
mod repo_context_artifact;
mod run_args;
mod runtime;

use crate::cli_command::Cli;
use crate::commands::Commands;
use crate::runtime::{run_config_canonicalize, run_config_validate, run_orchestrator};
use clap::Parser;
use std::process;

fn main() {
    let raw_args: Vec<String> = std::env::args().skip(1).collect();
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Fixture { args }) => fixture::run(&args),
        Some(Commands::LinkedStack { args }) => match linked_stack::run(&args) {
            Ok(()) => process::exit(0),
            Err(error) => {
                eprintln!("{error}");
                process::exit(1);
            }
        },
        Some(Commands::ConfigValidate(args)) => run_config_validate(args),
        Some(Commands::ConfigCanonicalize(args)) => run_config_canonicalize(args),
        None => run_orchestrator(cli.run, &raw_args),
    }
}
