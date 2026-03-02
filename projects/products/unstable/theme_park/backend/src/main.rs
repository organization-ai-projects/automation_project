mod config;
mod determinism;
mod diagnostics;
mod economy;
mod events;
mod io;
mod map;
mod protocol;
mod public_api;
mod replay;
mod report;
mod reputation;
mod rides;
mod routing;
mod scenario;
mod shops;
mod sim;
mod snapshot;
mod time;
mod visitors;

use crate::public_api::RequestDispatcher;
use std::io::BufRead;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args[1] != "serve" {
        eprintln!("Usage: theme_park_backend serve --scenario <file>");
        std::process::exit(2);
    }

    let mut scenario_path: Option<String> = None;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--scenario" => {
                i += 1;
                if i < args.len() {
                    scenario_path = Some(args[i].clone());
                } else {
                    eprintln!("--scenario requires a value");
                    std::process::exit(2);
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let mut dispatcher = RequestDispatcher::new(scenario_path);
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("IO error reading stdin: {}", e);
                std::process::exit(5);
            }
        };
        if line.trim().is_empty() {
            continue;
        }
        let response = dispatcher.dispatch(&line);
        println!("{}", response);
        if dispatcher.should_shutdown() {
            break;
        }
    }
}
