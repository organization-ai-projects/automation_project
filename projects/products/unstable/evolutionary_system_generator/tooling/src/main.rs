// Tooling entry point

mod diagnostics;
mod public_api;
mod validate;

use std::env;

use crate::validate::determinism_validator::{DeterminismValidator, ValidatorConfig};
use crate::validate::replay_validator::ReplayValidator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: evo-tooling <command> [options]");
        eprintln!("Commands:");
        eprintln!("  validate-determinism --seed <N> --generations <N>");
        eprintln!("  validate-replay --seed <N> --generations <N> --replay-path <path>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "validate-determinism" => {
            let seed = parse_flag(&args, "--seed").unwrap_or(42);
            let generations = parse_flag(&args, "--generations").unwrap_or(5);
            let backend_bin =
                parse_str_flag(&args, "--backend-bin").unwrap_or_else(|| "evo-backend".to_string());
            let config = ValidatorConfig {
                seed,
                population_size: 10,
                max_generations: generations as u32,
                rule_pool: default_rule_pool(),
            };
            match DeterminismValidator::validate(config, &backend_bin) {
                Ok(result) => {
                    println!(
                        "Determinism check: {}",
                        if result.determinism_ok {
                            "PASS"
                        } else {
                            "FAIL"
                        }
                    );
                    if let Some(hash) = result.manifest_hash {
                        println!("Manifest hash: {}", hash);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "validate-replay" => {
            let seed = parse_flag(&args, "--seed").unwrap_or(42);
            let generations = parse_flag(&args, "--generations").unwrap_or(5);
            let replay_path = parse_str_flag(&args, "--replay-path")
                .unwrap_or_else(|| "/tmp/replay.json".to_string());
            let backend_bin =
                parse_str_flag(&args, "--backend-bin").unwrap_or_else(|| "evo-backend".to_string());
            let config = ValidatorConfig {
                seed,
                population_size: 10,
                max_generations: generations as u32,
                rule_pool: default_rule_pool(),
            };
            match ReplayValidator::validate(config, &replay_path, &backend_bin) {
                Ok(result) => {
                    println!(
                        "Replay check: {}",
                        if result.replay_ok { "PASS" } else { "FAIL" }
                    );
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        cmd => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
    }
}

fn parse_flag(args: &[String], flag: &str) -> Option<u64> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .and_then(|w| w[1].parse().ok())
}

fn parse_str_flag(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

fn default_rule_pool() -> Vec<String> {
    vec![
        "rule_a".to_string(),
        "rule_b".to_string(),
        "rule_c".to_string(),
        "rule_d".to_string(),
    ]
}
