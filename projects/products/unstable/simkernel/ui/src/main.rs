mod app;
mod diagnostics;
mod fixtures;
mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::public_api::Controller;

fn main() {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(2);
    }

    match args[1].as_str() {
        "run" => {
            let mut pack = String::new();
            let mut seed: u64 = 0;
            let mut ticks: u64 = 100;
            let mut out = String::from("report.json");
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--pack" => { i += 1; if i < args.len() { pack = args[i].clone(); } }
                    "--seed" => { i += 1; if i < args.len() { seed = args[i].parse().unwrap_or(0); } }
                    "--ticks" => { i += 1; if i < args.len() { ticks = args[i].parse().unwrap_or(100); } }
                    "--out" => { i += 1; if i < args.len() { out = args[i].clone(); } }
                    _ => {}
                }
                i += 1;
            }
            let mut controller = Controller::new();
            match controller.run_pack(&pack, seed, ticks, &out) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(5);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(2);
        }
    }
}

fn print_usage() {
    println!("simkernel_ui - deterministic simulation UI client");
    println!();
    println!("Commands:");
    println!("  run --pack <pack_kind> --seed <n> --ticks <n> --out <file>");
}
