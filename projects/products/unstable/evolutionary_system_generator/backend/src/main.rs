// Backend entry point: REPL loop reading JSON from stdin, writing JSON to stdout

mod constraints;
mod diagnostics;
mod evaluate;
mod genome;
mod io;
mod output;
mod protocol;
mod public_api;
mod replay;
mod search;
mod seed;
mod tooling;

#[cfg(test)]
mod tests;

use std::io::BufRead;
use std::{env, process};

use crate::protocol::message::{read_request, write_response};
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::replay::event_log::EventLog;
use crate::replay::replay_engine::ReplayEngine;
use crate::search::evolution_engine::{EvolutionEngine, SearchConfig};
use crate::seed::seed::Seed;
use crate::tooling::determinism_validator::{DeterminismValidator, ValidatorConfig};
use crate::tooling::replay_validator::ReplayValidator;

fn dispatch(engine: &mut Option<EvolutionEngine>, request: Request) -> Response {
    match request {
        Request::NewSearch {
            seed,
            population_size,
            max_generations,
            rule_pool,
            constraints,
        } => {
            let config = SearchConfig {
                seed: Seed(seed),
                population_size,
                max_generations,
                rule_pool,
                constraints,
            };
            *engine = Some(EvolutionEngine::new(config));
            Response::Ok
        }
        Request::StepGen => match engine {
            None => Response::Error {
                message: "No search active".to_string(),
            },
            Some(e) => {
                let done = e.step_generation();
                let pop = e.get_population();
                let best = pop
                    .individuals
                    .iter()
                    .map(|i| i.fitness.0)
                    .fold(f64::NEG_INFINITY, f64::max);
                Response::Report {
                    generation: pop.generation,
                    best_fitness: best,
                    population_size: pop.individuals.len(),
                    done,
                }
            }
        },
        Request::RunToEnd => match engine {
            None => Response::Error {
                message: "No search active".to_string(),
            },
            Some(e) => {
                e.run_to_end();
                let pop = e.get_population();
                let best = pop
                    .individuals
                    .iter()
                    .map(|i| i.fitness.0)
                    .fold(f64::NEG_INFINITY, f64::max);
                Response::Report {
                    generation: pop.generation,
                    best_fitness: best,
                    population_size: pop.individuals.len(),
                    done: true,
                }
            }
        },
        Request::GetCandidates { top_n } => match engine {
            None => Response::Error {
                message: "No search active".to_string(),
            },
            Some(e) => {
                let manifest = e.build_candidate_manifest(top_n);
                Response::Candidates { manifest }
            }
        },
        Request::SaveReplay { path } => match engine {
            None => Response::Error {
                message: "No search active".to_string(),
            },
            Some(e) => match e.get_event_log().save_to_file(&path) {
                Ok(()) => Response::Ok,
                Err(err) => Response::Error {
                    message: err.to_string(),
                },
            },
        },
        Request::LoadReplay {
            path,
            rule_pool,
            constraints,
        } => match EventLog::load_from_file(&path) {
            Err(err) => Response::Error {
                message: err.to_string(),
            },
            Ok(log) => match ReplayEngine::replay_from_log(&log, rule_pool, constraints, 5) {
                Err(err) => Response::Error {
                    message: err.to_string(),
                },
                Ok(_result) => Response::Ok,
            },
        },
        Request::ReplayToEnd => match engine {
            None => Response::Error {
                message: "No search active".to_string(),
            },
            Some(e) => {
                let pop = e.get_population();
                let best = pop
                    .individuals
                    .iter()
                    .map(|i| i.fitness.0)
                    .fold(f64::NEG_INFINITY, f64::max);
                Response::Report {
                    generation: pop.generation,
                    best_fitness: best,
                    population_size: pop.individuals.len(),
                    done: true,
                }
            }
        },
    }
}

fn print_response(resp: &Response) {
    println!("{}", write_response(resp));
}

fn parse_u64_flag(args: &[String], flag: &str) -> Option<u64> {
    args.windows(2)
        .find(|window| window[0] == flag)
        .and_then(|window| window[1].parse().ok())
}

fn parse_str_flag(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].clone())
}

fn default_rule_pool() -> Vec<String> {
    vec![
        "rule_a".to_string(),
        "rule_b".to_string(),
        "rule_c".to_string(),
        "rule_d".to_string(),
    ]
}

fn run_tooling_command(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("Usage: evo-backend <command> [options]");
        eprintln!("Commands:");
        eprintln!("  validate-determinism --seed <N> --generations <N>");
        eprintln!("  validate-replay --seed <N> --generations <N> --replay-path <path>");
        return 2;
    }

    let backend_bin = parse_str_flag(args, "--backend-bin").unwrap_or_else(|| "evo-backend".into());
    match args[0].as_str() {
        "validate-determinism" => {
            let seed = parse_u64_flag(args, "--seed").unwrap_or(42);
            let generations = parse_u64_flag(args, "--generations").unwrap_or(5);
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
                        println!("Manifest hash: {hash}");
                    }
                    0
                }
                Err(err) => {
                    eprintln!("Error: {err}");
                    1
                }
            }
        }
        "validate-replay" => {
            let seed = parse_u64_flag(args, "--seed").unwrap_or(42);
            let generations = parse_u64_flag(args, "--generations").unwrap_or(5);
            let replay_path =
                parse_str_flag(args, "--replay-path").unwrap_or_else(|| "/tmp/replay.json".into());
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
                    0
                }
                Err(err) => {
                    eprintln!("Error: {err}");
                    1
                }
            }
        }
        command => {
            eprintln!("Unknown command: {command}");
            2
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        process::exit(run_tooling_command(&args[1..]));
    }

    let stdin = std::io::stdin();
    let mut engine: Option<EvolutionEngine> = None;

    for line in stdin.lock().lines() {
        let line = line.expect("stdin read error");
        if line.trim().is_empty() {
            continue;
        }

        let request = match read_request(&line) {
            Ok(r) => r,
            Err(e) => {
                print_response(&Response::Error { message: e });
                continue;
            }
        };

        let response = dispatch(&mut engine, request);
        print_response(&response);
    }
}
