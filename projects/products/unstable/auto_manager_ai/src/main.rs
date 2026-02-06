// projects/products/unstable/auto_manager_ai/src/main.rs

mod domain;
mod adapters;
mod ai;
mod config;
mod plan_generator;
mod plan_evaluator;
mod output_writer;

use std::env;
use std::path::PathBuf;
use std::process;

use config::Config;
use domain::{ActionPlan, RunReport};
use plan_generator::generate_action_plan;
use plan_evaluator::evaluate_plan;
use output_writer::write_outputs;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    let repo_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    };

    let output_dir = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from("./out")
    };

    println!("Auto Manager AI V0 (Safe-by-Default)");
    println!("Repository: {:?}", repo_path);
    println!("Output: {:?}", output_dir);
    println!();

    // Create configuration
    let config = Config::new(repo_path, output_dir.clone());

    // Generate run ID
    let run_id = format!("run_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());

    // Create run report
    let mut report = RunReport::new(run_id);

    // Generate action plan
    println!("Generating action plan...");
    let plan = match generate_action_plan(&config) {
        Ok(plan) => {
            report.output.actions_proposed = plan.actions.len();
            println!("Generated {} actions", plan.actions.len());
            plan
        }
        Err(e) => {
            eprintln!("Error generating action plan: {}", e);
            report.add_error(e);
            if let Err(write_err) = write_outputs(&ActionPlan::new("Error".to_string()), &report, &config.output_dir) {
                eprintln!("Failed to write error report: {}", write_err);
            }
            process::exit(1);
        }
    };

    // Evaluate plan against policy
    println!("Evaluating actions against policy...");
    let decisions = evaluate_plan(&plan, &config.policy);
    
    for decision in decisions {
        report.add_decision(decision);
    }

    println!("Policy evaluation complete:");
    println!("  Allowed: {}", report.output.actions_allowed);
    println!("  Denied: {}", report.output.actions_denied);
    println!("  Needs input: {}", report.output.actions_needs_input);
    println!();

    // Write outputs
    println!("Writing outputs to {:?}...", config.output_dir);
    if let Err(e) = write_outputs(&plan, &report, &config.output_dir) {
        eprintln!("Error writing outputs: {}", e);
        report.add_error(e);
        process::exit(1);
    }

    println!("Done! Outputs written to:");
    println!("  - {:?}/action_plan.json", config.output_dir);
    println!("  - {:?}/run_report.json", config.output_dir);
    println!();
    println!("V0 Note: All actions are proposals only. No mutations were performed.");
}
