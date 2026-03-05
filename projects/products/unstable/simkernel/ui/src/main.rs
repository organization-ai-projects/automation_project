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
            let mut replay_out: Option<String> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--pack" => {
                        i += 1;
                        if i < args.len() {
                            pack = args[i].clone();
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
                            ticks = args[i].parse().unwrap_or(100);
                        }
                    }
                    "--out" => {
                        i += 1;
                        if i < args.len() {
                            out = args[i].clone();
                        }
                    }
                    "--replay-out" => {
                        i += 1;
                        if i < args.len() {
                            replay_out = Some(args[i].clone());
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            let mut controller = Controller::new();
            match controller.run_pack_with_replay(&pack, seed, ticks, &out, replay_out.as_deref()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(5);
                }
            }
        }
        "replay" => {
            let mut replay = String::new();
            let mut out = String::from("report.json");
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--replay" => {
                        i += 1;
                        if i < args.len() {
                            replay = args[i].clone();
                        }
                    }
                    "--out" => {
                        i += 1;
                        if i < args.len() {
                            out = args[i].clone();
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            if replay.is_empty() {
                eprintln!("Missing --replay <file>");
                std::process::exit(2);
            }
            let mut controller = Controller::new();
            if let Err(e) = controller.replay_to_report(&replay, &out) {
                eprintln!("Error: {}", e);
                std::process::exit(4);
            }
        }
        "inspect" => {
            let mut replay = String::new();
            let mut query = String::new();
            let mut as_json = false;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--replay" => {
                        i += 1;
                        if i < args.len() {
                            replay = args[i].clone();
                        }
                    }
                    "--query" => {
                        i += 1;
                        if i < args.len() {
                            query = args[i].clone();
                        }
                    }
                    "--json" => {
                        as_json = true;
                    }
                    _ => {}
                }
                i += 1;
            }
            if replay.is_empty() || query.is_empty() {
                eprintln!("Missing --replay <file> or --query <q>");
                std::process::exit(2);
            }
            let mut controller = Controller::new();
            match controller.inspect_replay(&replay, &query) {
                Ok(result) => {
                    if as_json {
                        println!("{}", result);
                    } else {
                        println!("Inspect result: {}", result);
                    }
                }
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
    println!("  run --pack <pack_kind> --seed <n> --ticks <n> --out <file> [--replay-out <file>]");
    println!("  replay --replay <replay.bin> --out <report.json>");
    println!("  inspect --replay <replay.bin> --query <q> --json");
}
