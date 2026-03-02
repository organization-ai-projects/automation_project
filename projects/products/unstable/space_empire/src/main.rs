mod build;
mod combat;
mod config;
mod diagnostics;
mod economy;
mod events;
mod io;
mod model;
mod replay;
mod report;
mod research;
mod scenario;
mod ships;
mod snapshot;
mod time;
mod travel;

use std::collections::{BTreeMap, BTreeSet};

use build::BuildQueue;
use config::SimConfig;
use diagnostics::SpaceEmpireError;
use economy::{EconomyEngine, ResourceWallet};
use events::{EventLog, SimEvent};
use model::{EmpireState, PlanetState, SimState};
use report::{RunHash, SimReport, TickReport};
use research::{ResearchEngine, ResearchQueue};
use scenario::{Scenario, ScenarioLoader};
use snapshot::{SnapshotHash, StateSnapshot};
use time::{Tick, TickClock};
use travel::TravelEngine;

struct Sim {
    config: SimConfig,
    scenario: Scenario,
    state: SimState,
    clock: TickClock,
    event_log: EventLog,
    travel_engine: TravelEngine,
}

impl Sim {
    fn new(config: SimConfig, scenario: Scenario) -> Self {
        let mut empires = BTreeMap::new();
        for empire_setup in &scenario.empires {
            let mut planets = BTreeMap::new();
            for planet_setup in &empire_setup.planets {
                planets.insert(
                    planet_setup.planet_id,
                    PlanetState {
                        planet_id: planet_setup.planet_id,
                        empire_id: empire_setup.empire_id,
                        building_levels: planet_setup.building_levels.clone(),
                        production_modifier: 1.0,
                    },
                );
            }
            let mut wallet = ResourceWallet::new();
            for (&resource, &amount) in &empire_setup.starting_resources {
                wallet.add(resource, amount);
            }
            empires.insert(
                empire_setup.empire_id,
                EmpireState {
                    empire_id: empire_setup.empire_id,
                    planets,
                    wallet,
                    build_queue: BuildQueue::new(),
                    research_queue: ResearchQueue::new(),
                    fleets: BTreeMap::new(),
                    researched_techs: BTreeSet::new(),
                },
            );
        }

        let state = SimState {
            tick: Tick(0),
            empires,
        };

        Self {
            config,
            scenario,
            state,
            clock: TickClock::new(),
            event_log: EventLog::new(),
            travel_engine: TravelEngine::new(),
        }
    }

    fn run(&mut self) -> Result<SimReport, SpaceEmpireError> {
        let scenario_hash = ScenarioLoader::scenario_hash(&self.scenario);
        let mut tick_reports = Vec::new();
        let ticks = self.config.ticks;

        for i in 0..ticks {
            self.tick_once();
            let current_tick = self.clock.current();

            if i % 100 == 0 || i == ticks - 1 {
                let snapshot = StateSnapshot::from_state(&self.state, current_tick)?;
                let snapshot_hash = SnapshotHash::compute(&snapshot);
                let event_count = self.event_log.len();
                tick_reports.push(TickReport {
                    tick: current_tick,
                    snapshot_hash,
                    event_count,
                });
            }
        }

        let final_snapshot = StateSnapshot::from_state(&self.state, self.clock.current())?;
        let final_snapshot_hash = SnapshotHash::compute(&final_snapshot);
        let total_events = self.event_log.len();

        let report_summary = format!(
            "{{\"seed\":{},\"ticks_run\":{},\"scenario_hash\":\"{}\",\"total_events\":{}}}",
            self.config.seed, ticks, scenario_hash, total_events
        );
        let run_hash = RunHash::compute(&report_summary);

        let replay_file = replay::ReplayFile {
            seed: self.config.seed,
            scenario_hash: scenario_hash.clone(),
            ticks_run: ticks,
            events: self.event_log.events.clone(),
        };
        let _ = replay::ReplayCodec::encode(&replay_file);

        Ok(SimReport {
            seed: self.config.seed,
            ticks_run: ticks,
            scenario_hash,
            tick_reports,
            final_snapshot_hash,
            run_hash,
            total_events,
        })
    }

    fn tick_once(&mut self) {
        self.clock.advance();
        let tick = self.clock.current();
        self.state.tick = tick;

        let mut tick_events: Vec<SimEvent> = Vec::new();
        EconomyEngine::tick(&mut self.state, &mut tick_events);
        build::BuildEngine::tick(&mut self.state, tick, &mut tick_events);
        ResearchEngine::tick(&mut self.state, tick, &mut tick_events);
        self.travel_engine
            .tick(&mut self.state, tick, &mut tick_events);

        for event in tick_events {
            self.event_log.push(event);
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: space_empire <command> [options]");
        eprintln!("Commands: run, replay, report");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "replay" => cmd_replay(&args[2..]),
        "report" => cmd_report(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}

fn parse_flag<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].as_str())
}

fn cmd_run(args: &[String]) {
    let ticks: u64 = parse_flag(args, "--ticks")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);
    let seed: u64 = parse_flag(args, "--seed")
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);
    let scenario_path = parse_flag(args, "--scenario").unwrap_or("").to_string();
    let out_dir = parse_flag(args, "--out").unwrap_or(".").to_string();

    let scenario = if scenario_path.is_empty() {
        ScenarioLoader::default_scenario()
    } else {
        match ScenarioLoader::load_from_json(&scenario_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to load scenario: {e}");
                std::process::exit(1);
            }
        }
    };

    let config = SimConfig {
        seed,
        ticks,
        scenario_path,
    };
    let mut sim = Sim::new(config, scenario);

    match sim.run() {
        Ok(report) => {
            tracing::info!(
                "Simulation complete. {} events, {} ticks.",
                report.total_events,
                report.ticks_run
            );
            let out_path = format!("{out_dir}/sim_report.json");
            let json = format!(
                "{{\"seed\":{},\"ticks_run\":{},\"scenario_hash\":\"{}\",\"total_events\":{},\"run_hash\":\"{}\"}}",
                report.seed,
                report.ticks_run,
                report.scenario_hash,
                report.total_events,
                report.run_hash.0
            );
            if let Err(e) = std::fs::write(&out_path, json) {
                eprintln!("Failed to write report: {e}");
            } else {
                println!("Report written to {out_path}");
            }
        }
        Err(e) => {
            eprintln!("Simulation failed: {e}");
            std::process::exit(1);
        }
    }
}

fn cmd_replay(args: &[String]) {
    let replay_path = match parse_flag(args, "--replay") {
        Some(p) => p.to_string(),
        None => {
            eprintln!("--replay <file> required");
            std::process::exit(1);
        }
    };
    let scenario_path = parse_flag(args, "--scenario").unwrap_or("").to_string();

    let replay_bytes = match std::fs::read(&replay_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to read replay file: {e}");
            std::process::exit(1);
        }
    };

    let replay_file = match replay::ReplayCodec::decode(&replay_bytes) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to decode replay: {e}");
            std::process::exit(1);
        }
    };

    let scenario = if scenario_path.is_empty() {
        ScenarioLoader::default_scenario()
    } else {
        match ScenarioLoader::load_from_json(&scenario_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to load scenario: {e}");
                std::process::exit(1);
            }
        }
    };

    let config = SimConfig {
        seed: replay_file.seed,
        ticks: replay_file.ticks_run,
        scenario_path,
    };

    match replay::ReplayEngine::replay(&replay_file, &scenario, &config) {
        Ok(report) => {
            println!(
                "Replay complete. {} events, {} ticks.",
                report.total_events, report.ticks_run
            );
        }
        Err(e) => {
            eprintln!("Replay failed: {e}");
            std::process::exit(1);
        }
    }
}

fn cmd_report(args: &[String]) {
    let in_path = match parse_flag(args, "--in") {
        Some(p) => p.to_string(),
        None => {
            eprintln!("--in <sim_report.json> required");
            std::process::exit(1);
        }
    };

    match std::fs::read_to_string(&in_path) {
        Ok(content) => {
            println!("Report contents:\n{content}");
        }
        Err(e) => {
            eprintln!("Failed to read report: {e}");
            std::process::exit(1);
        }
    }
}
