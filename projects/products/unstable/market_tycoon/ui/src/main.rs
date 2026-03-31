//! projects/products/unstable/market_tycoon/ui/src/main.rs
mod app;
mod components;
mod screens;
mod state;
mod transport;

#[cfg(test)]
mod tests;

use crate::app::app;
use crate::screens::{ReplayScreen, RunScreen, ScenarioScreen};
use dioxus::launch;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("help");

    let result: Result<(), String> = match command {
        "run" => RunScreen::execute(args.get(2..).unwrap_or_default()).map_err(|e| e.to_string()),
        "replay" => {
            ReplayScreen::execute(args.get(2..).unwrap_or_default()).map_err(|e| e.to_string())
        }
        "scenario" => {
            ScenarioScreen::execute(args.get(2..).unwrap_or_default()).map_err(|e| e.to_string())
        }
        "ui" => {
            launch(app);
            Ok(())
        }
        _ => {
            eprintln!("Usage: market_tycoon_ui <run|replay|scenario|ui> [OPTIONS]");
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
