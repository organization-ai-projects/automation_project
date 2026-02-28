mod commands;
mod config;
mod determinism;
mod diagnostics;
mod ecs;
mod events;
mod inspect;
mod io;
mod packs;
mod packs_builtin;
mod protocol;
mod public_api;
mod replay;
mod report;
mod scenario;
mod schedule;
mod snapshot;
mod time;

use crate::public_api::RequestDispatcher;
use std::io::BufRead;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args[1] != "serve" {
        eprintln!("Usage: simkernel_backend serve [--pack <pack_kind>] [--scenario <file>]");
        std::process::exit(2);
    }

    let mut dispatcher = RequestDispatcher::new();
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
