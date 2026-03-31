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
    let mut application = app::App::new();
    application.update_status("Application started".to_string());
    println!("Status: {:?}", application.state().status());

    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("help");

    let result = match command {
        "run" => RunScreen::execute(&args[2..]),
        "replay" => ReplayScreen::execute(&args[2..]),
        "scenario" => ScenarioScreen::execute(&args[2..]),
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
