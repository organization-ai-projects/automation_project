use auto_render::cli::{CliError, ExplainCommand, PlanCommand, ReplayCommand};
use std::path::PathBuf;

fn main() -> Result<(), CliError> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "plan" => {
            if args.len() < 3 {
                return Err(CliError::Parse("Usage: plan <intent.ron> [--out <out.ron>] [--trace <trace.json>]".to_string()));
            }
            let mut intent_paths = vec![];
            let mut out_path = PathBuf::from("plan_out.ron");
            let mut trace_path = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--out" => {
                        i += 1;
                        if i < args.len() {
                            out_path = PathBuf::from(&args[i]);
                        }
                    }
                    "--trace" => {
                        i += 1;
                        if i < args.len() {
                            trace_path = Some(PathBuf::from(&args[i]));
                        }
                    }
                    path => intent_paths.push(PathBuf::from(path)),
                }
                i += 1;
            }
            PlanCommand { intent_paths, out_path, trace_path }.run()
        }
        "replay" => {
            if args.len() < 3 {
                return Err(CliError::Parse("Usage: replay <plan.ron> [--fingerprint]".to_string()));
            }
            let plan_path = PathBuf::from(&args[2]);
            let print_fingerprint = args.iter().any(|a| a == "--fingerprint");
            ReplayCommand { plan_path, print_fingerprint }.run()
        }
        "explain" => {
            if args.len() < 3 {
                return Err(CliError::Parse("Usage: explain <plan.ron>".to_string()));
            }
            let plan_path = PathBuf::from(&args[2]);
            ExplainCommand { plan_path }.run()
        }
        _ => Err(CliError::NoSuchCommand),
    }
}

fn print_usage() {
    println!("auto_render - cinematography rendering engine");
    println!();
    println!("Commands:");
    println!("  plan <intent.ron> [--out <out.ron>] [--trace <trace.json>]");
    println!("  replay <plan.ron> [--fingerprint]");
    println!("  explain <plan.ron>");
}
