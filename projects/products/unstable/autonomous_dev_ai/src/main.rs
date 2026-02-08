// projects/products/unstable/autonomous_dev_ai/src/main.rs

use autonomous_dev_ai::{AutonomousAgent, load_config, save_ron};
use std::env;
use std::process;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <goal> [config_path] [audit_log]", args[0]);
        eprintln!("\nExample:");
        eprintln!(
            "  {} \"Fix the failing tests\" ./agent_config ./audit.log",
            args[0]
        );
        process::exit(1);
    }

    let goal = &args[1];
    let config_path = if args.len() > 2 {
        args[2].clone()
    } else {
        "./agent_config".to_string()
    };
    let audit_log = if args.len() > 3 {
        args[3].clone()
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
            println!("✓ Config loaded: {}", config.agent_name);
            println!("  Execution mode: {}", config.execution_mode);
            println!("  Neural enabled: {}", config.neural.enabled);
            println!("  Objectives: {}", config.objectives.len());
        }
        Err(_) => {
            println!("Config not found, creating default configuration...");
            let config = autonomous_dev_ai::config::AgentConfig::default();
            if let Err(e) = save_ron(format!("{}.ron", config_path), &config) {
                eprintln!("Failed to create config: {}", e);
                process::exit(1);
            }
            println!("✓ Created default config at {}.ron", config_path);
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

    println!("\n🚀 Starting agent execution...\n");

    // Run agent
    match agent.run(goal) {
        Ok(()) => {
            println!("\n✓ Agent completed successfully");

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
            eprintln!("\n✗ Agent failed: {}", e);

            // Try to save state even on failure
            let _ = agent.save_state();

            process::exit(1);
        }
    }
}
