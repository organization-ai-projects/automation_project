// projects/products/unstable/city_builder/ui/src/main.rs
#[cfg(target_arch = "wasm32")]
mod web_app;

#[cfg(not(target_arch = "wasm32"))]
mod app;
#[cfg(not(target_arch = "wasm32"))]
mod diagnostics;
#[cfg(not(target_arch = "wasm32"))]
mod screens;
#[cfg(not(target_arch = "wasm32"))]
mod transport;
#[cfg(not(target_arch = "wasm32"))]
mod widgets;

#[cfg(not(target_arch = "wasm32"))]
use app::cli::Cli;
#[cfg(not(target_arch = "wasm32"))]
use app::runner;
#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(_) => std::process::exit(2),
    };

    let code = match cli.command {
        app::app_command::AppCommand::Run {
            ticks,
            seed,
            scenario,
            out,
            replay_out,
        } => runner::run_cli(
            ticks,
            seed,
            scenario.as_path(),
            out.as_path(),
            replay_out.as_deref(),
        ),
        app::app_command::AppCommand::Replay { replay, out } => {
            runner::replay_cli(replay.as_path(), out.as_path())
        }
        app::app_command::AppCommand::Snapshot {
            replay,
            at_tick,
            out,
        } => runner::snapshot_cli(replay.as_path(), at_tick, out.as_path()),
        app::app_command::AppCommand::Validate { scenario } => {
            runner::validate_cli(scenario.as_path())
        }
    };

    match code {
        Ok(c) => std::process::exit(c),
        Err(_) => std::process::exit(5),
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(web_app::web_app);
}
