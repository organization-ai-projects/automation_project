// projects/products/unstable/hospital_tycoon/backend/src/protocol/server.rs
use crate::diagnostics::app_error::AppError;
use crate::io::json_codec::JsonCodec;
use crate::public_api::BackendApi;
use std::io::BufRead;
use std::path::PathBuf;

pub fn run(args: &[String]) -> i32 {
    if args.len() < 2 || args[1] != "serve" {
        print_usage();
        return 2;
    }

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

    let scenario_path = match scenario_path {
        Some(path) => path,
        None => {
            eprintln!("error: --scenario <file> required");
            return 2;
        }
    };

    if !scenario_path.exists() {
        eprintln!(
            "error: scenario file not found: {}",
            scenario_path.display()
        );
        return 3;
    }

    let mut api = match BackendApi::from_scenario_path(scenario_path) {
        Ok(value) => value,
        Err(AppError::Config(error)) => {
            eprintln!("error: invalid scenario: {}", error);
            return 3;
        }
        Err(error) => {
            eprintln!("error: {}", error);
            return 5;
        }
    };

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(value) => value,
            Err(error) => {
                eprintln!("error: {}", error);
                return 5;
            }
        };
        if line.trim().is_empty() {
            continue;
        }
        let response = api.handle_line(&line);
        match JsonCodec::encode_response(&response) {
            Ok(encoded) => println!("{}", encoded),
            Err(error) => {
                eprintln!("error: {}", error);
                return 5;
            }
        }
    }

    0
}

fn print_usage() {
    eprintln!("hospital_tycoon_backend - deterministic hospital simulation backend");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  serve --scenario <file.json>");
    eprintln!();
    eprintln!("Exit codes: 0=clean, 2=invalid CLI, 3=invalid scenario, 5=internal error");
}
