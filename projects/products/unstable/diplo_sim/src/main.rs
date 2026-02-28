use diplo_sim::config;
use diplo_sim::diagnostics::error::DiploSimError;

fn print_usage() {
    eprintln!("Usage:");
    eprintln!(
        "  diplo_sim run --turns N --seed S --map <map_file> --players <n> --out <match.json> [--replay-out <replay.bin>]"
    );
    eprintln!("  diplo_sim replay --replay <replay.bin> --out <match.json>");
    eprintln!("  diplo_sim validate-map --map <map_file>");
    eprintln!("  diplo_sim validate-orders --map <map_file> --orders <orders_file>");
}

fn get_arg(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(2);
    }

    let result = match args[1].as_str() {
        "run" => {
            let turns: u32 = get_arg(&args, "--turns")
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| {
                    eprintln!("Missing --turns");
                    std::process::exit(2);
                });
            let seed: u64 = get_arg(&args, "--seed")
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| {
                    eprintln!("Missing --seed");
                    std::process::exit(2);
                });
            let map_path = get_arg(&args, "--map").unwrap_or_else(|| {
                eprintln!("Missing --map");
                std::process::exit(2);
            });
            let players: u32 = get_arg(&args, "--players")
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| {
                    eprintln!("Missing --players");
                    std::process::exit(2);
                });
            let out = get_arg(&args, "--out").unwrap_or_else(|| {
                eprintln!("Missing --out");
                std::process::exit(2);
            });
            let replay_out = get_arg(&args, "--replay-out");

            config::runner::run_simulation(
                turns,
                seed,
                &map_path,
                players,
                &out,
                replay_out.as_deref(),
            )
        }
        "replay" => {
            let replay_path = get_arg(&args, "--replay").unwrap_or_else(|| {
                eprintln!("Missing --replay");
                std::process::exit(2);
            });
            let out = get_arg(&args, "--out").unwrap_or_else(|| {
                eprintln!("Missing --out");
                std::process::exit(2);
            });
            config::runner::replay_simulation(&replay_path, &out)
        }
        "validate-map" => {
            let map_path = get_arg(&args, "--map").unwrap_or_else(|| {
                eprintln!("Missing --map");
                std::process::exit(2);
            });
            config::runner::validate_map(&map_path)
        }
        "validate-orders" => {
            let map_path = get_arg(&args, "--map").unwrap_or_else(|| {
                eprintln!("Missing --map");
                std::process::exit(2);
            });
            let orders_path = get_arg(&args, "--orders").unwrap_or_else(|| {
                eprintln!("Missing --orders");
                std::process::exit(2);
            });
            config::runner::validate_orders_cmd(&map_path, &orders_path)
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            std::process::exit(2);
        }
    };

    match result {
        Ok(()) => {}
        Err(DiploSimError::Io(msg)) => {
            eprintln!("IO error: {}", msg);
            std::process::exit(3);
        }
        Err(DiploSimError::MapValidation(msg)) => {
            eprintln!("Map validation error: {}", msg);
            std::process::exit(3);
        }
        Err(DiploSimError::OrderValidation { .. }) => {
            eprintln!("Order validation error");
            std::process::exit(3);
        }
        Err(DiploSimError::Replay(msg)) => {
            eprintln!("Replay error: {}", msg);
            std::process::exit(4);
        }
        Err(DiploSimError::Internal(msg)) => {
            eprintln!("Internal error: {}", msg);
            std::process::exit(5);
        }
        Err(DiploSimError::Config(msg)) => {
            eprintln!("Config error: {}", msg);
            std::process::exit(2);
        }
    }
}
