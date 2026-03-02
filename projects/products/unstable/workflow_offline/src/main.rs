mod cli;
mod config;
mod diagnostics;
mod engine;
mod exec;
mod logging;
mod public_api;

use cli::args::parse_args;
use config::workflow_config::WorkflowConfig;
use engine::workflow_engine::WorkflowEngine;
use std::process;

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("error: {msg}");
            process::exit(2);
        }
    };

    let config = match WorkflowConfig::from_file(&args.workflow_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(e.exit_code());
        }
    };

    let engine = WorkflowEngine::new(config, args.seed, args.dry_run);

    match engine.run() {
        Ok(report) => {
            if args.json {
                match serde_json::to_string_pretty(&report) {
                    Ok(json) => println!("{json}"),
                    Err(e) => {
                        eprintln!("error serialising report: {e}");
                        process::exit(4);
                    }
                }
            } else if !args.dry_run {
                println!(
                    "workflow `{}` completed successfully ({} job(s))",
                    report.workflow_name,
                    report.jobs.len()
                );
            }
            process::exit(0);
        }
        Err(e) => {
            eprintln!("error: {e}");
            if args.json {
                // Emit a minimal error JSON so callers can still parse stdout.
                let err_json = serde_json::json!({ "error": e.to_string() });
                eprintln!("{err_json}");
            }
            process::exit(e.exit_code());
        }
    }
}
