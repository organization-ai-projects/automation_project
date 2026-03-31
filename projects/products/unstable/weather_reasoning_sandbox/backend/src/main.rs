mod application;
mod cli;
mod domain;
mod infrastructure;
mod replay;
mod reporting;
mod simulation;

#[cfg(test)]
mod tests;

use std::process;

use crate::application::export_report::ExportReport;
use crate::application::export_snapshot::ExportSnapshot;
use crate::application::replay_simulation::ReplaySimulation;
use crate::application::run_simulation::RunSimulation;
use crate::cli::cli_error::CliError;
use crate::cli::cli_parser::{CliCommand, CliParser};
use crate::cli::output_mode::OutputMode;
use crate::infrastructure::journal_persistence::JournalPersistence;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let result = match CliParser::parse(&args) {
        Ok(command) => execute(command),
        Err(e) => {
            eprintln!("Error: {e}");
            print_usage();
            process::exit(2);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn execute(command: CliCommand) -> Result<(), CliError> {
    match command {
        CliCommand::Run(args) => {
            let output = RunSimulation::execute(args.seed, args.ticks, args.dataset.as_deref())
                .map_err(CliError::RunError)?;

            let journal_path = format!(
                "weather_reasoning_sandbox_journal_s{}_t{}.json",
                args.seed, args.ticks
            );
            JournalPersistence::save(&journal_path, &output.journal)
                .map_err(CliError::ExportError)?;
            eprintln!("Journal written to {journal_path}");

            match args.output_mode {
                OutputMode::Json => {
                    let json =
                        ExportReport::to_json(&output.report).map_err(CliError::ExportError)?;
                    println!("{json}");
                }
                OutputMode::Text => {
                    println!("Run complete.");
                    println!("  seed: {}", output.report.metadata.seed);
                    println!("  ticks: {}", output.report.metadata.tick_count);
                    println!("  dataset: {}", output.report.metadata.dataset);
                    println!("  contradictions: {}", output.report.contradiction_count);
                    println!("  total violations: {}", output.report.total_violations);
                    println!("  total corrections: {}", output.report.total_corrections);
                    println!("  report checksum: {}", output.report.report_checksum);
                    if let Some(ref sc) = output.report.snapshot_checksum {
                        println!("  snapshot checksum: {sc}");
                    }
                }
            }

            Ok(())
        }
        CliCommand::Replay(args) => {
            let output = ReplaySimulation::execute(&args.replay_file)
                .map_err(|e| CliError::ReplayError(e.to_string()))?;

            match args.output_mode {
                OutputMode::Json => {
                    let json =
                        ExportReport::to_json(&output.report).map_err(CliError::ExportError)?;
                    println!("{json}");
                }
                OutputMode::Text => {
                    println!("Replay complete.");
                    println!("  seed: {}", output.report.metadata.seed);
                    println!("  ticks: {}", output.report.metadata.tick_count);
                    println!("  equivalent: {}", output.replay_result.is_equivalent);
                    println!(
                        "  replay report checksum: {}",
                        output.replay_result.replay_report_checksum
                    );
                    println!(
                        "  replay snapshot checksum: {}",
                        output.replay_result.replay_snapshot_checksum
                    );
                }
            }

            Ok(())
        }
    }
}

fn print_usage() {
    eprintln!("Usage: weather_reasoning_sandbox <command> [options]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  run     --ticks N --seed S [--dataset <path>] [--json]");
    eprintln!("  replay  <replay_file> [--json]");
}
