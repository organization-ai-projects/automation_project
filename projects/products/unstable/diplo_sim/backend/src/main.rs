mod adjudication;
mod ai;
mod cli_args;
mod config;
mod diagnostics;
mod io;
mod map;
mod model;
mod orders;
mod replay;
mod report;
#[cfg(test)]
mod tests;
mod time;

use crate::diagnostics::diplo_sim_error::DiploSimError;

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        cli_args::print_usage();
        std::process::exit(2);
    }

    let result = match args[1].as_str() {
        "run" => {
            let turns: u32 = cli_args::get_arg(&args, "--turns")
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| {
                    tracing::error!("Missing --turns");
                    std::process::exit(2);
                });
            let seed: u64 = cli_args::get_arg(&args, "--seed")
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| {
                    tracing::error!("Missing --seed");
                    std::process::exit(2);
                });
            let map_path = cli_args::get_arg(&args, "--map").unwrap_or_else(|| {
                tracing::error!("Missing --map");
                std::process::exit(2);
            });
            let players: u32 = cli_args::get_arg(&args, "--players")
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(|| {
                    tracing::error!("Missing --players");
                    std::process::exit(2);
                });
            let out = cli_args::get_arg(&args, "--out").unwrap_or_else(|| {
                tracing::error!("Missing --out");
                std::process::exit(2);
            });
            let replay_out = cli_args::get_arg(&args, "--replay-out");

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
            let replay_path = cli_args::get_arg(&args, "--replay").unwrap_or_else(|| {
                tracing::error!("Missing --replay");
                std::process::exit(2);
            });
            let out = cli_args::get_arg(&args, "--out").unwrap_or_else(|| {
                tracing::error!("Missing --out");
                std::process::exit(2);
            });
            config::runner::replay_simulation(&replay_path, &out)
        }
        "validate-map" => {
            let map_path = cli_args::get_arg(&args, "--map").unwrap_or_else(|| {
                tracing::error!("Missing --map");
                std::process::exit(2);
            });
            config::runner::validate_map(&map_path)
        }
        "validate-orders" => {
            let map_path = cli_args::get_arg(&args, "--map").unwrap_or_else(|| {
                tracing::error!("Missing --map");
                std::process::exit(2);
            });
            let orders_path = cli_args::get_arg(&args, "--orders").unwrap_or_else(|| {
                tracing::error!("Missing --orders");
                std::process::exit(2);
            });
            config::runner::validate_orders_cmd(&map_path, &orders_path)
        }
        _ => {
            tracing::error!("Unknown command: {}", args[1]);
            cli_args::print_usage();
            std::process::exit(2);
        }
    };

    match result {
        Ok(()) => {}
        Err(DiploSimError::Io(msg)) => {
            tracing::error!("IO error: {}", msg);
            std::process::exit(3);
        }
        Err(DiploSimError::MapValidation(msg)) => {
            tracing::error!("Map validation error: {}", msg);
            std::process::exit(3);
        }
        Err(DiploSimError::OrderValidation { .. }) => {
            tracing::error!("Order validation error");
            std::process::exit(3);
        }
        Err(DiploSimError::Replay(msg)) => {
            tracing::error!("Replay error: {}", msg);
            std::process::exit(4);
        }
        Err(DiploSimError::Internal(msg)) => {
            tracing::error!("Internal error: {}", msg);
            std::process::exit(5);
        }
        Err(DiploSimError::Config(msg)) => {
            tracing::error!("Config error: {}", msg);
            std::process::exit(2);
        }
    }
}
