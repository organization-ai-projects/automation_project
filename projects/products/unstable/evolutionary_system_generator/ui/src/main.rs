// projects/products/unstable/evolutionary_system_generator/ui/src/main.rs

mod app;
mod diagnostics;
mod screens;
mod transport;
mod widgets;

use std::io::{BufRead, Write};

use app::action::Action;
use app::app_state::AppState;
use app::controller::Controller;
use app::screen::Screen;
use common_json::Json;
use screens::candidate_screen::render_candidate_screen;
use screens::config_screen::render_config_screen;
use screens::population_screen::render_population_screen;
use screens::report_screen::render_report_screen;
use screens::run_screen::render_run_screen;
use widgets::plot_widget::render_plot;
use widgets::table_widget::render_table;

fn main() {
    let mut state = AppState::default();
    let mut controller = Controller::new();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    println!("=== Evolutionary System Generator UI ===");
    println!("Commands: new-search, step, run, candidates, quit");

    let mut out = stdout.lock();
    for line in render_config_screen() {
        if writeln!(out, "{line}").is_err() {
            return;
        }
    }

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
                for response_line in responses {
                    if writeln!(out, "{response_line}").is_err() {
                        return;
                    }
                }
                for screen_line in render_state_snapshot(&state) {
                    if writeln!(out, "{screen_line}").is_err() {
                        return;
                    }
                }
            }
            None => {
                if writeln!(out, "Unknown command: {}", trimmed).is_err() {
                    return;
                }
            }
        }
    }
}

fn render_state_snapshot(state: &AppState) -> Vec<String> {
    match state.current_screen {
        Screen::Config => render_config_screen(),
        Screen::Running => {
            let mut lines = render_run_screen(state.generation, state.best_fitness, state.done);
            lines.extend(render_population_screen(state.generation as usize + 1));
            lines.extend(render_plot(&[state.best_fitness]));
            if state.done {
                lines.extend(render_report_screen(state.generation, "n/a"));
            }
            lines
        }
        Screen::Candidates => {
            let mut rows: Vec<Vec<String>> = Vec::new();
            if let Some(manifest) = &state.last_manifest {
                let candidates = json_field(manifest, "manifest")
                    .and_then(|m| json_field(m, "candidates"))
                    .and_then(Json::as_array);
                if let Some(candidates) = candidates {
                    for (idx, candidate) in candidates.iter().take(5).enumerate() {
                        let fitness = json_field(candidate, "fitness")
                            .and_then(Json::as_f64)
                            .unwrap_or(0.0);
                        rows.push(vec![(idx + 1).to_string(), format!("{fitness:.4}")]);
                    }
                }
            }
            let mut lines = render_table(&["Rank", "Fitness"], &rows);
            if rows.is_empty() {
                lines.push("No candidates available.".to_string());
            } else {
                for row in &rows {
                    if let Some(fitness) = row.get(1).and_then(|v| v.parse::<f64>().ok()) {
                        lines.extend(render_candidate_screen(
                            row[0].parse::<usize>().unwrap_or(0),
                            fitness,
                        ));
                    }
                }
            }
            lines
        }
        Screen::Report => render_report_screen(state.generation, "n/a"),
    }
}

fn json_field<'a>(json: &'a Json, key: &str) -> Option<&'a Json> {
    json.as_object().and_then(|obj| obj.get(key))
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
