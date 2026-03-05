// projects/products/unstable/digital_pet/ui/src/main.rs
mod app;
mod diagnostics;
mod fixtures;
pub mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::diagnostics::app_error::AppError;
use crate::public_api::UiApi;
use std::path::PathBuf;

fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    match args[1].as_str() {
        "run" => {
            let mut scenario: Option<PathBuf> = None;
            let mut seed: u64 = 42;
            let mut ticks: u64 = 100;
            let mut out: Option<PathBuf> = None;
            let mut replay_out: Option<PathBuf> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--scenario" => {
                        i += 1;
                        if i < args.len() {
                            scenario = Some(PathBuf::from(&args[i]));
                        }
                    }
                    "--seed" => {
                        i += 1;
                        if i < args.len() {
                            seed = args[i].parse().unwrap_or(42);
                        }
                    }
                    "--ticks" => {
                        i += 1;
                        if i < args.len() {
                            ticks = args[i].parse().unwrap_or(100);
                        }
                    }
                    "--out" => {
                        i += 1;
                        if i < args.len() {
                            out = Some(PathBuf::from(&args[i]));
                        }
                    }
                    "--replay-out" => {
                        i += 1;
                        if i < args.len() {
                            replay_out = Some(PathBuf::from(&args[i]));
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            let scenario = scenario.unwrap_or_else(|| PathBuf::from("scenario.json"));
            UiApi::run(scenario, seed, ticks, out, replay_out)
        }
        "replay" => {
            let mut replay: Option<PathBuf> = None;
            let mut out: Option<PathBuf> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--replay" => {
                        i += 1;
                        if i < args.len() {
                            replay = Some(PathBuf::from(&args[i]));
                        }
                    }
                    "--out" => {
                        i += 1;
                        if i < args.len() {
                            out = Some(PathBuf::from(&args[i]));
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            let replay =
                replay.ok_or_else(|| AppError::Process("missing --replay <file>".to_string()))?;
            let out =
                out.ok_or_else(|| AppError::Process("missing --out <report.json>".to_string()))?;
            UiApi::replay(replay, out)
        }
        _ => {
            print_usage();
            Ok(())
        }
    }
}

fn print_usage() {
    println!("digital_pet_ui - deterministic digital pet UI");
    println!();
    println!("Commands:");
    println!(
        "  run --scenario <file> --seed S --ticks N --out <report.json> [--replay-out <file>]"
    );
    println!("  replay --replay <file> --out <report.json>");
}
