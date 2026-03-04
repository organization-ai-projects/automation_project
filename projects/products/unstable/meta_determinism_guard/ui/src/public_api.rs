// projects/products/unstable/meta_determinism_guard/ui/src/public_api.rs
use crate::app::action::Action;
use crate::app::app_state::AppState;
use crate::app::controller;
use crate::diagnostics::ui_error::UiError;

pub fn run_cli(args: &[String]) -> Result<(), UiError> {
    if args.len() < 2 {
        eprintln!("Usage: meta_determinism_guard_ui <command> [options]");
        eprintln!("Commands: scan, canon, stability, report");
        return Ok(());
    }

    let mut state = AppState::new();
    let action = parse_action(args)?;
    controller::dispatch(&mut state, action)?;
    Ok(())
}

fn parse_action(args: &[String]) -> Result<Action, UiError> {
    match args[1].as_str() {
        "scan" => {
            let root = find_flag(args, "--root").unwrap_or(".".to_string());
            let json = args.contains(&"--json".to_string());
            Ok(Action::Scan { root, json })
        }
        "canon" => {
            let path = find_flag(args, "--path")
                .ok_or_else(|| UiError::InvalidUsage("--path required".to_string()))?;
            let json = args.contains(&"--json".to_string());
            Ok(Action::Canon { path, json })
        }
        "stability" => {
            let cmd = find_flag(args, "--cmd")
                .ok_or_else(|| UiError::InvalidUsage("--cmd required".to_string()))?;
            let runs: u32 = find_flag(args, "--runs")
                .and_then(|s| s.parse().ok())
                .unwrap_or(3);
            let json = args.contains(&"--json".to_string());
            Ok(Action::Stability { cmd, runs, json })
        }
        "report" => {
            let json = args.contains(&"--json".to_string());
            Ok(Action::Report { json })
        }
        other => Err(UiError::InvalidUsage(format!("Unknown command: {}", other))),
    }
}

fn find_flag(args: &[String], flag: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == flag)?;
    args.get(pos + 1).cloned()
}
