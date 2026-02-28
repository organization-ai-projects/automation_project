pub mod public_api;

mod app;
mod diagnostics;
mod screens;
mod transport;
mod widgets;

use std::io::{BufRead, Write};

use app::action::Action;
use app::app_state::AppState;
use app::controller::Controller;

fn main() {
    let mut state = AppState::default();
    let mut controller = Controller::new();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    println!("=== Evolutionary System Generator UI ===");
    println!("Commands: new-search, step, run, candidates, quit");

    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let action = parse_command(trimmed);
        match action {
            Some(Action::Quit) => break,
            Some(act) => {
                let responses = controller.handle(&mut state, act);
                for r in responses {
                    writeln!(out, "{}", r).unwrap();
                }
            }
            None => {
                writeln!(out, "Unknown command: {}", trimmed).unwrap();
            }
        }
    }
}

fn parse_command(input: &str) -> Option<Action> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    match parts[0] {
        "new-search" => Some(Action::StartSearch {
            seed: 42,
            population_size: 10,
            max_generations: 5,
            rule_pool: vec![
                "rule_a".to_string(),
                "rule_b".to_string(),
                "rule_c".to_string(),
            ],
        }),
        "step" => Some(Action::StepGen),
        "run" => Some(Action::RunToEnd),
        "candidates" => Some(Action::ShowCandidates { top_n: 5 }),
        "quit" => Some(Action::Quit),
        _ => None,
    }
}
