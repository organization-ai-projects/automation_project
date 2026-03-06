mod application;
mod buildings;
mod citizens;
mod config;
mod diagnostics;
mod economy;
mod events;
mod io;
mod map;
mod replay;
mod report;
mod scenarios;
mod services;
mod snapshot;
mod time;
mod traffic;
mod zoning;

use application::app_command::AppCommand;
use application::cli::Cli;
use clap::Parser;
use diagnostics::city_builder_error::CityBuilderError;

fn main() {
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(_) => std::process::exit(2),
    };

    let result = match cli.command {
        AppCommand::Run {
            ticks,
            seed,
            scenario,
            out,
            replay_out,
        } => application::runner::run_cli(
            ticks,
            seed,
            scenario.as_path(),
            out.as_path(),
            replay_out.as_deref(),
        ),
        AppCommand::Replay { replay, out } => {
            application::runner::replay_cli(replay.as_path(), out.as_path())
        }
        AppCommand::Snapshot {
            replay,
            at_tick,
            out,
        } => application::runner::snapshot_cli(replay.as_path(), at_tick, out.as_path()),
        AppCommand::Validate { scenario } => application::runner::validate_cli(scenario.as_path()),
    };

    match result {
        Ok(()) => std::process::exit(0),
        Err(CityBuilderError::InvalidScenario(_)) | Err(CityBuilderError::InvalidConfig(_)) => {
            std::process::exit(3)
        }
        Err(CityBuilderError::ReplayMismatch(_)) => std::process::exit(4),
        Err(_) => std::process::exit(5),
    }
}
