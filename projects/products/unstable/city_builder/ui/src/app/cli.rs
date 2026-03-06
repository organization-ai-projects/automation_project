use clap::Parser;

#[derive(Parser)]
#[command(name = "city_builder_ui")]
pub struct Cli {
    #[command(subcommand)]
    pub command: super::app_command::AppCommand,
}
