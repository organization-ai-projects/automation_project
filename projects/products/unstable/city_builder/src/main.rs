mod buildings;
mod citizens;
mod config;
mod diagnostics;
mod economy;
mod events;
mod io;
mod map;
mod public_api;
mod replay;
mod report;
mod scenario;
mod services;
mod snapshot;
mod time;
mod traffic;
mod zoning;

use clap::{Parser, Subcommand};
use diagnostics::error::CityBuilderError;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "city_builder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(long)]
        ticks: u64,
        #[arg(long)]
        seed: u64,
        #[arg(long)]
        scenario: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
}

fn main() {
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    match cli.command {
        Commands::Run {
            ticks,
            seed,
            scenario,
            out,
        } => match run_sim(ticks, seed, &scenario, &out) {
            Ok(()) => std::process::exit(0),
            Err(CityBuilderError::InvalidScenario(_)) | Err(CityBuilderError::InvalidConfig(_)) => {
                eprintln!("Error: invalid scenario or config");
                std::process::exit(3);
            }
            Err(CityBuilderError::ReplayMismatch(_)) => {
                eprintln!("Error: replay mismatch");
                std::process::exit(4);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(5);
            }
        },
    }
}

fn run_sim(
    ticks: u64,
    seed: u64,
    scenario_path: &PathBuf,
    out_path: &PathBuf,
) -> Result<(), CityBuilderError> {
    use buildings::growth_engine::GrowthEngine;
    use config::sim_config::SimConfig;
    use economy::economy_engine::EconomyEngine;
    use io::json_codec::JsonCodec;
    use report::run_hash::RunHash;
    use report::sim_report::SimReport;
    use report::tick_report::TickReport;
    use scenario::scenario_loader::ScenarioLoader;
    use services::service_engine::ServiceEngine;
    use snapshot::state_snapshot::StateSnapshot;
    use traffic::traffic_engine::TrafficEngine;

    let scenario = ScenarioLoader::load(scenario_path)?;

    let config = SimConfig {
        grid_width: scenario.grid_width,
        grid_height: scenario.grid_height,
        seed,
        total_ticks: ticks,
    };

    let mut state = StateSnapshot::from_scenario(&scenario, &config);
    let mut tick_reports: Vec<TickReport> = Vec::new();

    let mut growth = GrowthEngine::new();
    let mut service = ServiceEngine::new();
    let mut traffic_eng = TrafficEngine::new();
    let mut economy = EconomyEngine::new();

    for t in 1..=ticks {
        let tick = time::tick::Tick(t);
        growth.tick(&mut state, tick, &config);
        service.tick(&mut state);
        traffic_eng.tick(&mut state);
        economy.tick(&mut state, &config);

        let hash = snapshot::snapshot_hash::SnapshotHash::compute(&state);
        let report = TickReport {
            tick: t,
            building_count: state.buildings.len() as u32,
            total_population: state.total_population(),
            budget_balance: state.budget_balance,
            snapshot_hash: hash.value.clone(),
        };
        tick_reports.push(report);

        for cp in &scenario.checkpoints {
            if cp.tick == t && cp.expected_hash != hash.value {
                return Err(CityBuilderError::ReplayMismatch(format!(
                    "Checkpoint at tick {t}: expected {}, got {}",
                    cp.expected_hash, hash.value
                )));
            }
        }
    }

    let run_hash = RunHash::compute(&scenario.name, seed, ticks, &tick_reports);

    let sim_report = SimReport {
        scenario_name: scenario.name.clone(),
        seed,
        total_ticks: ticks,
        tick_reports,
        run_hash: run_hash.value,
    };

    JsonCodec::write(&sim_report, out_path)?;
    Ok(())
}
