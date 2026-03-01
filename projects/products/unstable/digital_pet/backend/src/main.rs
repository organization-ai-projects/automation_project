// projects/products/unstable/digital_pet/backend/src/main.rs
mod battle;
mod care;
mod config;
mod diagnostics;
mod events;
mod evolution;
mod io;
mod model;
mod needs;
mod protocol;
pub mod public_api;
mod replay;
mod report;
mod scenario;
mod snapshot;
mod time;
mod training;

use crate::diagnostics::error::AppError;
use crate::public_api::BackendApi;
use std::path::PathBuf;

fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
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
            let path =
                scenario_path.ok_or_else(|| AppError::Config("--scenario required".into()))?;
            BackendApi::serve(path)
        }
        _ => {
            print_usage();
            Ok(())
        }
    }
}

fn print_usage() {
    println!("digital_pet_backend - deterministic digital pet backend");
    println!();
    println!("Commands:");
    println!("  serve --scenario <file.json>");
}
