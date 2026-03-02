mod actions;
mod config;
mod decision;
mod diagnostics;
mod interaction;
mod io;
mod model;
mod needs;
mod public_api;
mod relations;
mod replay;
mod report;
mod schedule;
mod sim;
mod time;
mod traits;

use crate::config::SimConfig;
use crate::diagnostics::LifeSimError;
use crate::io::JsonCodec;
use crate::model::agent_id::AgentId;
use crate::model::object_id::ObjectId;
use crate::model::room_id::RoomId;
use crate::model::{Agent, Object, Room, World};
use crate::needs::NeedsState;
use crate::relations::MemoryLog;
use crate::replay::{ReplayCodec, ReplayEngine, ReplayFile};
use crate::report::RunReport;
use crate::schedule::Schedule;
use crate::sim::event_log::EventLog;
use crate::sim::sim_engine::SimEngine;
use crate::sim::sim_state::SimState;
use crate::time::{Tick, TickClock};
use crate::traits::TraitProfile;
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let result = match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "replay" => cmd_replay(&args[2..]),
        "report" => cmd_report(&args[2..]),
        _ => Err(LifeSimError::NoSuchCommand),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  life_sim run --ticks N --seed S [--scenario <file>] [--out <dir>]");
    println!("  life_sim replay --input <replay.json> [--out <dir>]");
    println!("  life_sim report --input <run_report.json>");
}

fn cmd_run(args: &[String]) -> Result<(), LifeSimError> {
    let mut ticks: u64 = 100;
    let mut seed: u64 = 42;
    let mut scenario_path: Option<String> = None;
    let mut output_path: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--ticks" => {
                i += 1;
                ticks = args
                    .get(i)
                    .and_then(|s| s.parse().ok())
                    .ok_or_else(|| LifeSimError::Config("Invalid ticks value".into()))?;
            }
            "--seed" => {
                i += 1;
                seed = args
                    .get(i)
                    .and_then(|s| s.parse().ok())
                    .ok_or_else(|| LifeSimError::Config("Invalid seed value".into()))?;
            }
            "--scenario" => {
                i += 1;
                scenario_path = args.get(i).cloned();
            }
            "--out" => {
                i += 1;
                output_path = args.get(i).cloned();
            }
            _ => {}
        }
        i += 1;
    }

    let config = SimConfig {
        ticks,
        seed,
        scenario_path: scenario_path.clone(),
        output_path: output_path.clone(),
    };

    let world = if let Some(ref path) = scenario_path {
        JsonCodec::read_file::<World>(Path::new(path))?
    } else {
        default_world()
    };

    let mut state = SimState {
        world: world.clone(),
        clock: TickClock::new(Tick(0)),
        event_log: EventLog::default(),
        seed,
    };

    let engine = SimEngine::new(config.clone());
    let report = engine.run(&mut state)?;

    println!("Run complete. Hash: {}", report.run_hash.0);

    if let Some(ref out) = output_path {
        let out_dir = Path::new(out);
        std::fs::create_dir_all(out_dir).map_err(|e| LifeSimError::Io(e.to_string()))?;

        JsonCodec::write_file(&out_dir.join("run_report.json"), &report)?;

        let replay = ReplayFile {
            seed,
            config,
            initial_world: world,
            events: state.event_log.events,
        };
        JsonCodec::write_file(&out_dir.join("replay.json"), &replay)?;
    }

    Ok(())
}

fn cmd_replay(args: &[String]) -> Result<(), LifeSimError> {
    let mut input: Option<String> = None;
    let mut output_path: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                i += 1;
                input = args.get(i).cloned();
            }
            "--out" => {
                i += 1;
                output_path = args.get(i).cloned();
            }
            _ => {}
        }
        i += 1;
    }

    let input_path = input.ok_or_else(|| LifeSimError::Config("--input required".into()))?;
    let content =
        std::fs::read_to_string(&input_path).map_err(|e| LifeSimError::Io(e.to_string()))?;
    let replay_file = ReplayCodec::deserialize(&content)?;

    let report = ReplayEngine::replay(&replay_file)?;
    println!("Replay complete. Hash: {}", report.run_hash.0);

    if let Some(ref out) = output_path {
        let out_dir = Path::new(out);
        std::fs::create_dir_all(out_dir).map_err(|e| LifeSimError::Io(e.to_string()))?;
        JsonCodec::write_file(&out_dir.join("run_report.json"), &report)?;
    }

    Ok(())
}

fn cmd_report(args: &[String]) -> Result<(), LifeSimError> {
    let mut input: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        if args[i].as_str() == "--input" {
            i += 1;
            input = args.get(i).cloned();
        }
        i += 1;
    }

    let input_path = input.ok_or_else(|| LifeSimError::Config("--input required".into()))?;
    let report: RunReport = JsonCodec::read_file(Path::new(&input_path))?;

    println!("Run Report");
    println!("  Seed: {}", report.seed);
    println!("  Ticks: {}", report.ticks_simulated);
    println!("  Hash: {}", report.run_hash.0);
    println!("  Agents: {}", report.agents.len());
    for agent in &report.agents {
        println!(
            "    - {} (id={}): actions={}",
            agent.name, agent.agent_id.0, agent.actions_taken
        );
    }

    Ok(())
}

fn default_world() -> World {
    let room1_id = RoomId(1);
    let room2_id = RoomId(2);
    let obj1_id = ObjectId(1);
    let obj2_id = ObjectId(2);
    let obj3_id = ObjectId(3);
    let agent1_id = AgentId(1);
    let agent2_id = AgentId(2);

    let mut rooms = BTreeMap::new();
    rooms.insert(
        room1_id,
        Room {
            id: room1_id,
            name: "Living Room".into(),
            objects: vec![obj1_id, obj2_id],
            capacity: 10,
        },
    );
    rooms.insert(
        room2_id,
        Room {
            id: room2_id,
            name: "Kitchen".into(),
            objects: vec![obj3_id],
            capacity: 5,
        },
    );

    let mut objects = BTreeMap::new();
    objects.insert(
        obj1_id,
        Object {
            id: obj1_id,
            name: "Television".into(),
            room: room1_id,
            tags: vec!["entertainment".into()],
        },
    );
    objects.insert(
        obj2_id,
        Object {
            id: obj2_id,
            name: "Sofa".into(),
            room: room1_id,
            tags: vec!["comfort".into()],
        },
    );
    objects.insert(
        obj3_id,
        Object {
            id: obj3_id,
            name: "Refrigerator".into(),
            room: room2_id,
            tags: vec!["food".into()],
        },
    );

    let mut agents = BTreeMap::new();
    agents.insert(
        agent1_id,
        Agent {
            id: agent1_id,
            name: "Alice".into(),
            room: room1_id,
            needs: NeedsState::new_full(),
            traits: TraitProfile::new(vec![]),
            schedule: Schedule::default(),
            relationships: BTreeMap::new(),
            memory: MemoryLog::default(),
        },
    );
    agents.insert(
        agent2_id,
        Agent {
            id: agent2_id,
            name: "Bob".into(),
            room: room1_id,
            needs: NeedsState::new_full(),
            traits: TraitProfile::new(vec![]),
            schedule: Schedule::default(),
            relationships: BTreeMap::new(),
            memory: MemoryLog::default(),
        },
    );

    World {
        rooms,
        objects,
        agents,
    }
}
