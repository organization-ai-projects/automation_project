// projects/products/unstable/auto_manager_ai/src/main.rs

mod domain;
mod adapters;
mod ai;

use std::env;
use std::path::{Path, PathBuf};
use std::process;
use std::fs;
use common_json::to_string_pretty;

use domain::{ActionPlan, Policy, PolicyDecision, RunReport};
use adapters::{RepoAdapter, GhAdapter, CiAdapter};
use ai::{Planner, PlanningContext};

/// Configuration for the automation manager
#[derive(Debug, Clone)]
struct Config {
    repo_path: PathBuf,
    output_dir: PathBuf,
    policy: Policy,
}

impl Config {
    /// Create a new configuration
    fn new(repo_path: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            repo_path,
            output_dir,
            policy: Policy::default(),
        }
    }
}

/// Generate an action plan based on repository and GitHub context
fn generate_action_plan(config: &Config) -> Result<ActionPlan, String> {
    // Create adapters
    let repo_adapter = RepoAdapter::new(config.repo_path.clone());
    let gh_adapter = GhAdapter::new();
    let ci_adapter = CiAdapter::new();

    // Gather context
    let repo_ctx = repo_adapter.get_context()?;
    let gh_ctx = gh_adapter.get_context()?;
    let ci_ctx = ci_adapter.get_context()?;

    // Create planning context
    let planning_ctx = PlanningContext {
        repo: repo_ctx,
        gh: gh_ctx,
        ci: ci_ctx,
    };

    // Generate plan
    Ok(Planner::generate_plan(&planning_ctx))
}

/// Evaluate an action plan against policy
fn evaluate_plan(plan: &ActionPlan, policy: &Policy) -> Vec<PolicyDecision> {
    plan.actions
        .iter()
        .map(|action| policy.evaluate(action))
        .collect()
}

/// Write outputs to the output directory
fn write_outputs(
    plan: &ActionPlan,
    report: &RunReport,
    out_dir: &Path,
) -> Result<(), String> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(out_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    // Write action plan
    let action_plan_path = out_dir.join("action_plan.json");
    let action_plan_json = to_string_pretty(plan)
        .map_err(|e| format!("Failed to serialize action plan: {:?}", e))?;
    fs::write(&action_plan_path, action_plan_json)
        .map_err(|e| format!("Failed to write action plan: {}", e))?;

    // Write run report
    let run_report_path = out_dir.join("run_report.json");
    let run_report_json = to_string_pretty(report)
        .map_err(|e| format!("Failed to serialize run report: {:?}", e))?;
    fs::write(&run_report_path, run_report_json)
        .map_err(|e| format!("Failed to write run report: {}", e))?;

    Ok(())
}

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
