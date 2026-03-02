// projects/products/unstable/hospital_tycoon/backend/src/main.rs
mod config;
mod diagnostics;
mod economy;
mod io;
mod model;
mod patients;
mod protocol;
pub mod public_api;
mod replay;
mod report;
mod reputation;
mod rooms;
mod sim;
mod snapshot;
mod staff;
mod time;
mod triage;

use crate::diagnostics::error::AppError;
use crate::public_api::BackendApi;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(0);
    }
    match args[1].as_str() {
        "serve" => {
            let mut scenario_path: Option<PathBuf> = None;
            let mut i = 2;
            while i < args.len() {
                if args[i] == "--scenario" {
                    i += 1;
                    if i < args.len() {
                        scenario_path = Some(PathBuf::from(&args[i]));
                    }
                }
                i += 1;
            }
            let path = match scenario_path {
                Some(p) => p,
                None => {
                    eprintln!("error: --scenario <file> required");
                    std::process::exit(2);
                }
            };
            if !path.exists() {
                eprintln!("error: scenario file not found: {}", path.display());
                std::process::exit(3);
            }
            match BackendApi::serve(path) {
                Ok(_) => std::process::exit(0),
                Err(AppError::Config(e)) => {
                    eprintln!("error: invalid scenario: {}", e);
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
    eprintln!("hospital_tycoon_backend - deterministic hospital simulation backend");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  serve --scenario <file.json>");
    eprintln!();
    eprintln!("Exit codes: 0=clean, 2=invalid CLI, 3=invalid scenario, 5=internal error");
}
