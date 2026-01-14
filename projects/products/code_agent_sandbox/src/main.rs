// ==========================
// Code Agent Sandbox - Main
// ==========================
//
// This file represents the main entry point of the Code Agent Sandbox application.
// It orchestrates the various steps required to execute actions, train the AI,
// and generate results. This file is structured to facilitate understanding and
// future extensions.
//
// File Path: projects/products/code_agent_sandbox/src/main.rs
//
// Main Steps:
// 1. Engine initialization
// 2. Execution of low-level actions
// 3. AI agent orchestration
// 4. Generation of scores and responses
// 5. Neural model training and saving
//
// ==========================

// Necessary module imports
mod access_kind;
mod actions;
mod agents;
mod command_runner;
mod engine;
mod execution_context;
mod execution_paths;
mod journal;
mod journal_line;
mod logging;
mod memory;
mod normalization;
mod policies;
mod runner_config;
mod sandbox_fs;
mod score;
mod worktree;

use anyhow::{Context, Result, bail};
use common_time::timeout::with_timeout; // Correction de l'import pour inclure `with_timeout`
use common_time::{SystemClock, TimeSpan};
use std::io::{self, Read};
use std::path::PathBuf;

use crate::engine::{EngineConfig, EnginePaths, Request};

fn main() -> Result<()> {
    // ==========================
    // Step 1: Argument Handling
    // ==========================
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <repo_root> <runs_root>",
            args.first()
                .map(|s| s.as_str())
                .unwrap_or("code_agent_sandbox")
        );
        bail!("invalid arguments");
    }

    let paths = EnginePaths {
        repo_root: PathBuf::from(&args[1]),
        runs_root: PathBuf::from(&args[2]),
    };

    let timeout = TimeSpan::from_secs(30);
    let config = EngineConfig::new(timeout);

    // ==========================
    // Step 2: Reading Standard Input
    // ==========================
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let req: Request = protocol::from_json_str(&input).context("Invalid JSON input for Request")?;

    // ==========================
    // Step 3: Execute Request
    // ==========================
    let clock = SystemClock; // Correction pour utiliser directement l'instance de SystemClock
    let timeout_future = with_timeout(
        engine::execute_request(req, &paths, config),
        &clock,
        timeout,
    );

    match futures::executor::block_on(timeout_future) {
        Ok(Ok(res)) => println!("{}", protocol::to_json_string_pretty(&res)?),
        Ok(Err(err)) => bail!("Execution error: {}", err),
        Err(_) => bail!("Timeout exceeded while executing request"),
    }

    Ok(())
}
