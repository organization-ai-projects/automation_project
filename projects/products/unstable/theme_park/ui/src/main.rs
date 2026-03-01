mod app;
mod diagnostics;
mod fixtures;
mod public_api;
mod screens;
mod transport;
mod widgets;

use crate::public_api::Controller;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(2);
    }

    match args[1].as_str() {
        "run" => {
            let mut scenario = String::new();
            let mut seed: u64 = 0;
            let mut ticks: u64 = 200;
            let mut out = String::from("report.json");
            let mut replay_out: Option<String> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--scenario" => {
                        i += 1;
                        if i < args.len() {
                            scenario = args[i].clone();
                        } else {
                            eprintln!("--scenario requires a value");
                            std::process::exit(2);
                        }
                    }
                    "--seed" => {
                        i += 1;
                        if i < args.len() {
                            seed = args[i].parse().unwrap_or(0);
                        }
                    }
                    "--ticks" => {
                        i += 1;
                        if i < args.len() {
                            ticks = args[i].parse().unwrap_or(200);
                        }
                    }
                    "--out" => {
                        i += 1;
                        if i < args.len() {
                            out = args[i].clone();
                        }
                    }
                    "--replay" => {
                        i += 1;
                        if i < args.len() {
                            replay_out = Some(args[i].clone());
                        }
                    }
                    _ => {
                        eprintln!("Unknown argument: {}", args[i]);
                        std::process::exit(2);
                    }
                }
                i += 1;
            }
            if scenario.is_empty() {
                eprintln!("--scenario is required");
                std::process::exit(2);
            }
            let mut controller = Controller::new();
            match controller.run(&scenario, seed, ticks, &out, replay_out.as_deref()) {
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
    println!("theme_park_ui - theme park management game UI client");
    println!();
    println!("Commands:");
    println!(
        "  run --scenario <file> --seed <n> --ticks <n> [--out <report.json>] [--replay <file>]"
    );
}
