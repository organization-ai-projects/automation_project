use clap::Parser;

use crate::application::app_command;

#[derive(Parser)]
#[command(name = "city_builder")]
pub struct Cli {
    #[command(subcommand)]
    pub command: app_command::AppCommand,
}
