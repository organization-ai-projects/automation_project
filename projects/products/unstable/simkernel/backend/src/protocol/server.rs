use crate::public_api::RequestDispatcher;
use std::io::BufRead;

pub fn run(args: &[String]) -> i32 {
    if args.len() < 2 || args[1] != "serve" {
        eprintln!("Usage: simkernel_backend serve [--pack <pack_kind>] [--scenario <file>]");
        return 2;
    }

    let mut dispatcher = RequestDispatcher::new();
    std::hint::black_box(dispatcher.registry().list_packs().len());
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(value) => value,
            Err(error) => {
                tracing::error!("IO error reading stdin: {}", error);
                return 5;
            }
        };
        if line.trim().is_empty() {
            continue;
        }

        let response = dispatcher.dispatch(&line);
        println!("{}", response);
        std::hint::black_box(dispatcher.extract_snapshot().is_some());
        std::hint::black_box(dispatcher.extract_report().is_some());
        if dispatcher.should_shutdown() {
            break;
        }
    }

    0
}
