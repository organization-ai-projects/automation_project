mod app;
mod components;
mod screens;
mod state;
mod transport;

#[cfg(test)]
mod tests;

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("help");

    let result = match command {
        "run" => screens::run_screen::RunScreen::execute(&args[2..]),
        "replay" => screens::replay_screen::ReplayScreen::execute(&args[2..]),
        "scenario" => screens::scenario_screen::ScenarioScreen::execute(&args[2..]),
        _ => {
            eprintln!("Usage: market_tycoon_ui <run|replay|scenario> [OPTIONS]");
            process::exit(2);
        }
    };

    match result {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    }
}
