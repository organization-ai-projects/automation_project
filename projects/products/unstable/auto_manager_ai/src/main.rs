// projects/products/unstable/auto_manager_ai/src/main.rs

mod adapters;
mod ai;
mod artifact_contract;
mod authz;
mod config;
mod domain;
mod engine_gateway;
mod executor;
mod output_writer;
mod plan_evaluator;
mod plan_generator;

#[cfg(test)]
mod tests;

use std::env;
use std::path::PathBuf;
use std::process;

use config::Config;
use domain::{ActionPlan, RunReport};
use engine_gateway::EngineGateway;
use executor::execute_allowed_actions;
use output_writer::write_outputs;
use plan_evaluator::evaluate_plan;
use plan_generator::generate_action_plan;

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

    println!("Auto Manager AI (Engine-Mediated, Safe-by-Default)");
    println!("Repository: {:?}", repo_path);
    println!("Output: {:?}", output_dir);
    println!();

    // Create configuration
    let config = Config::new(repo_path, output_dir.clone());

    // Generate run ID
    let run_id = format!(
        "run_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // Create run report
    let mut report = RunReport::new(run_id);
    report.record_lifecycle(format!("run_mode={:?}", config.run_mode));

    if let Err(authn_error) = authz::ensure_authenticated_actor(&config.actor) {
        eprintln!("Authentication failed: {authn_error}");
        report.add_error(authn_error);
        let _ = write_outputs(
            &ActionPlan::new("Authentication failed".to_string()),
            &report,
            &config.output_dir,
        );
        process::exit(1);
    }

    let engine = EngineGateway::new(config.run_mode);
    if let Err(e) = engine.register_startup() {
        eprintln!("Engine startup registration failed: {}", e.render());
        report.add_error(e.render());
        let _ = write_outputs(
            &ActionPlan::new("Engine startup registration failed".to_string()),
            &report,
            &config.output_dir,
        );
        process::exit(1);
    }
    if let Ok(event) = engine.record_health() {
        report.record_lifecycle(event);
    }

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
            if let Err(write_err) = write_outputs(
                &ActionPlan::new("Error".to_string()),
                &report,
                &config.output_dir,
            ) {
                eprintln!("Failed to write error report: {}", write_err);
            }
            process::exit(1);
        }
    };

    // Evaluate plan against policy
    println!("Evaluating actions against policy...");
    let decisions = evaluate_plan(&plan, &config.policy);

    for decision in decisions.clone() {
        report.add_decision(decision);
    }

    execute_allowed_actions(&plan, &decisions, &config, &mut report);

    println!("Policy evaluation complete:");
    println!("  Allowed: {}", report.output.actions_allowed);
    println!("  Denied: {}", report.output.actions_denied);
    println!("  Needs input: {}", report.output.actions_needs_input);
    println!("  Executed: {}", report.output.actions_executed);
    println!();

    if let Ok(event) = engine.register_shutdown() {
        report.record_lifecycle(event);
    }

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
    println!("Safety note: only low-risk allowlisted actions can execute.");
}
