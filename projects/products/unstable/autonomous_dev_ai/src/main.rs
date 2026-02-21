// projects/products/unstable/autonomous_dev_ai/src/main.rs
mod agent_config;
mod audit_event;
mod audit_logger;
mod autonomous_agent;
mod config_loader;
mod error;
mod lifecycle;
mod memory;
mod memory_graph;
mod neural;
mod neural_config;
mod objectif_score;
mod objective_evaluator;
mod objectives;
mod ops;
mod persistence;
mod plan_entry;
mod pr_flow;
mod security;
mod state;
mod symbolic;
mod symbolic_config;
mod tools;

use std::env;
use std::io::IsTerminal;
use std::process;

use crate::agent_config::AgentConfig;
use crate::autonomous_agent::AutonomousAgent;
use crate::config_loader::load_config;
use crate::persistence::save_ron;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let raw_args: Vec<String> = env::args().collect();
    let program_name = raw_args[0].clone();
    let mut args: Vec<String> = raw_args.into_iter().skip(1).collect();
    let pretty_requested = if let Some(pos) = args.iter().position(|arg| arg == "--pretty") {
        args.remove(pos);
        true
    } else {
        false
    };

    if args.is_empty() {
        eprintln!(
            "Usage: {} [--pretty] <goal> [config_path] [audit_log]",
            program_name
        );
        eprintln!("\nExample:");
        eprintln!(
            "  {} --pretty \"Fix the failing tests\" ./agent_config ./audit.log",
            program_name
        );
        process::exit(1);
    }

    let use_pretty_icons = pretty_requested && std::io::stdout().is_terminal();
    let ok_icon = if use_pretty_icons { "✓" } else { "[ok]" };
    let run_icon = if use_pretty_icons { "🚀" } else { "[run]" };
    let err_icon = if use_pretty_icons { "✗" } else { "[error]" };

    let goal = &args[0];
    let config_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "./agent_config".to_string()
    };
    let audit_log = if args.len() > 2 {
        args[2].clone()
    } else {
        "./agent_audit.log".to_string()
    };

    println!("========================================");
    println!("Autonomous Developer AI");
    println!("========================================");
    println!("Goal: {}", goal);
    println!("Config: {}", config_path);
    println!("Audit Log: {}", audit_log);
    println!("========================================\n");

    // Ensure config exists
    match load_config(&config_path) {
        Ok(config) => {
            println!("{ok_icon} Config loaded: {}", config.agent_name);
            println!("  Execution mode: {}", config.execution_mode);
            println!("  Neural enabled: {}", config.neural.enabled);
            println!("  Objectives: {}", config.objectives.len());
        }
        Err(_) => {
            println!("Config not found, creating default configuration...");
            let config = AgentConfig::default();
            if let Err(e) = save_ron(format!("{}.ron", config_path), &config) {
                eprintln!("Failed to create config: {}", e);
                process::exit(1);
            }
            println!("{ok_icon} Created default config at {}.ron", config_path);
        }
    }

    // Create agent
    let mut agent = match AutonomousAgent::new(&config_path, &audit_log) {
        Ok(agent) => agent,
        Err(e) => {
            eprintln!("Failed to create agent: {}", e);
            process::exit(1);
        }
    };

    println!("\n{run_icon} Starting agent execution...\n");

    // Run agent
    match agent.run(goal) {
        Ok(()) => {
            println!("\n{ok_icon} Agent completed successfully");

            // Save state
            if let Err(e) = agent.save_state() {
                eprintln!("Warning: Failed to save state: {}", e);
            }

            println!("\n========================================");
            println!("Execution Summary");
            println!("========================================");
            println!("Iterations: {}", agent.lifecycle.iteration);
            println!("Final state: {:?}", agent.lifecycle.state);
            println!(
                "Explored files: {}",
                agent.lifecycle.memory.explored_files.len()
            );
            println!("Decisions made: {}", agent.lifecycle.memory.decisions.len());
            println!("Failures: {}", agent.lifecycle.memory.failures.len());
            println!("========================================\n");

            println!("Audit log written to: {}", audit_log);
        }
        Err(e) => {
            eprintln!("\n{err_icon} Agent failed: {}", e);

            // Try to save state even on failure
            let _ = agent.save_state();

            process::exit(1);
        }
    }
}
