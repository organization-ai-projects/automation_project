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

#[cfg(test)]
mod tests;

use std::io::BufRead;

use crate::protocol::message::{read_request, write_response};
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::replay::event_log::EventLog;
use crate::replay::replay_engine::ReplayEngine;
use crate::search::evolution_engine::{EvolutionEngine, SearchConfig};
use crate::seed::seed::Seed;

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
            None => Response::Error { message: "No search active".to_string() },
            Some(e) => {
                let done = e.step_generation();
                let pop = e.get_population();
                let best = pop.individuals.iter()
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
            None => Response::Error { message: "No search active".to_string() },
            Some(e) => {
                e.run_to_end();
                let pop = e.get_population();
                let best = pop.individuals.iter()
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
            None => Response::Error { message: "No search active".to_string() },
            Some(e) => {
                let manifest = e.build_candidate_manifest(top_n);
                Response::Candidates { manifest }
            }
        },
        Request::SaveReplay { path } => match engine {
            None => Response::Error { message: "No search active".to_string() },
            Some(e) => {
                match e.get_event_log().save_to_file(&path) {
                    Ok(()) => Response::Ok,
                    Err(err) => Response::Error { message: err.to_string() },
                }
            }
        },
        Request::LoadReplay { path, rule_pool, constraints } => {
            match EventLog::load_from_file(&path) {
                Err(err) => Response::Error { message: err.to_string() },
                Ok(log) => {
                    match ReplayEngine::replay_from_log(&log, rule_pool, constraints, 5) {
                        Err(err) => Response::Error { message: err.to_string() },
                        Ok(_result) => Response::Ok,
                    }
                }
            }
        },
        Request::ReplayToEnd => match engine {
            None => Response::Error { message: "No search active".to_string() },
            Some(e) => {
                let pop = e.get_population();
                let best = pop.individuals.iter()
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

fn main() {
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
