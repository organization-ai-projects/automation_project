// projects/products/unstable/hospital_tycoon/ui/src/main.rs
mod app;
mod diagnostics;
mod fixtures;
pub mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::diagnostics::error::AppError;
use crate::public_api::UiApi;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(0);
    }
    match args[1].as_str() {
        "run" => {
            let mut scenario: Option<PathBuf> = None;
            let mut seed: u64 = 42;
            let mut ticks: u64 = 100;
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
                    _ => {}
                }
                i += 1;
            }
            let scenario = match scenario {
                Some(p) => p,
                None => {
                    eprintln!("error: --scenario <file> required");
                    std::process::exit(2);
                }
            };
            match UiApi::run(scenario, seed, ticks) {
                Ok(_) => std::process::exit(0),
                Err(AppError::Replay(e)) => {
                    eprintln!("error: {}", e);
                    std::process::exit(3);
                }
                Err(e) => {
                    eprintln!("error: {}", e);
                    std::process::exit(5);
                }
            }
        }
        "replay" => {
            let mut scenario: Option<PathBuf> = None;
            let mut replay: Option<PathBuf> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--scenario" => {
                        i += 1;
                        if i < args.len() {
                            scenario = Some(PathBuf::from(&args[i]));
                        }
                    }
                    "--replay" => {
                        i += 1;
                        if i < args.len() {
                            replay = Some(PathBuf::from(&args[i]));
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            let scenario = scenario.unwrap_or_else(|| PathBuf::from("scenario.json"));
            let replay = match replay {
                Some(p) => p,
                None => {
                    eprintln!("error: --replay <file> required");
                    std::process::exit(2);
                }
            };
            match UiApi::replay(scenario, replay) {
                Ok(_) => std::process::exit(0),
                Err(AppError::Replay(e)) => {
                    eprintln!("error: replay invalid: {}", e);
                    std::process::exit(3);
                }
                Err(e) => {
                    eprintln!("error: {}", e);
                    std::process::exit(5);
                }
            }
        }
        _ => {
            print_usage();
            std::process::exit(2);
        }
    }
}

fn print_usage() {
    eprintln!("hospital_tycoon_ui - hospital simulation UI");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  run --scenario <file> --seed S --ticks N");
    eprintln!("  replay --scenario <file> --replay <replay.json>");
    eprintln!();
    eprintln!("Exit codes: 0=success, 2=invalid CLI, 3=scenario/replay invalid, 5=internal error");
}
