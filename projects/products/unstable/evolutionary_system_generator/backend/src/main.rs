// projects/products/unstable/evolutionary_system_generator/backend/src/main.rs

mod constraints;
mod diagnostics;
mod evaluate;
mod genetics;
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

use crate::diagnostics::engine_error::EngineError;
use crate::protocol::message::{
    read_request, write_response, write_stderr_line, write_stdout_line,
};
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::replay::event_log::EventLog;
use crate::replay::replay_engine::ReplayEngine;
use crate::search::evolution_engine::EvolutionEngine;
use crate::search::search_config::SearchConfig;
use crate::seed::seed::Seed;
use crate::tooling::determinism_validator::DeterminismValidator;
use crate::tooling::replay_validator::ReplayValidator;
use crate::tooling::validator_config::ValidatorConfig;

fn engine_error_response(err: EngineError) -> Response {
    Response::Error {
        message: err.to_string(),
    }
}

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
            None => engine_error_response(EngineError::NoActiveSearch),
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
            None => engine_error_response(EngineError::NoActiveSearch),
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
            None => engine_error_response(EngineError::NoActiveSearch),
            Some(e) => {
                let manifest = e.build_candidate_manifest(top_n);
                Response::Candidates { manifest }
            }
        },
        Request::SaveReplay { path } => match engine {
            None => engine_error_response(EngineError::NoActiveSearch),
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
            Err(err) => engine_error_response(EngineError::Io(err)),
            Ok(log) => match ReplayEngine::replay_from_log(&log, rule_pool, constraints, 5) {
                Err(err) => engine_error_response(EngineError::Replay(err.to_string())),
                Ok(result) => {
                    if result.matches
                        && result.original_event_count == result.replayed_event_count
                        && result.original_event_count > 0
                    {
                        Response::Ok
                    } else {
                        Response::Error {
                            message: "Replay verification failed".to_string(),
                        }
                    }
                }
            },
        },
        Request::ReplayToEnd => match engine {
            None => engine_error_response(EngineError::NoActiveSearch),
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
    let line = write_response(resp);
    if write_stdout_line(&line).is_err() {
        process::exit(1);
    }
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
        if write_stderr_line("Usage: evo-backend <command> [options]").is_err() {
            return 1;
        }
        if write_stderr_line("Commands:").is_err() {
            return 1;
        }
        if write_stderr_line("  validate-determinism --seed <N> --generations <N>").is_err() {
            return 1;
        }
        if write_stderr_line("  validate-replay --seed <N> --generations <N> --replay-path <path>")
            .is_err()
        {
            return 1;
        }
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
                    if write_stdout_line(&format!(
                        "Determinism check: {}",
                        if result.determinism_ok {
                            "PASS"
                        } else {
                            "FAIL"
                        }
                    ))
                    .is_err()
                    {
                        return 1;
                    }
                    if let Some(hash) = result.manifest_hash {
                        if write_stdout_line(&format!("Manifest hash: {hash}")).is_err() {
                            return 1;
                        }
                    }
                    0
                }
                Err(err) => {
                    if write_stderr_line(&format!("Error: {err}")).is_err() {
                        return 1;
                    }
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
                    if write_stdout_line(&format!(
                        "Replay check: {}",
                        if result.replay_ok { "PASS" } else { "FAIL" }
                    ))
                    .is_err()
                    {
                        return 1;
                    }
                    0
                }
                Err(err) => {
                    if write_stderr_line(&format!("Error: {err}")).is_err() {
                        return 1;
                    }
                    1
                }
            }
        }
        command => {
            if write_stderr_line(&format!("Unknown command: {command}")).is_err() {
                return 1;
            }
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
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                print_response(&Response::Error {
                    message: format!("stdin read error: {err}"),
                });
                continue;
            }
        };
        if line.trim().is_empty() {
            continue;
        }

        let request = match read_request(&line) {
            Ok(r) => r,
            Err(e) => {
                print_response(&engine_error_response(EngineError::Serialization(e)));
                continue;
            }
        };

        let response = dispatch(&mut engine, request);
        print_response(&response);
    }
}
