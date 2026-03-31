mod config;
mod demand;
mod diagnostics;
mod events;
mod finance;
mod io;
mod logistics;
mod model;
mod pricing;
mod replay;
mod report;
mod scenario;
mod snapshot;
mod supply;
mod time;
mod transport;

#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::process;

use diagnostics::error::Error;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("");

    let result = match command {
        "run" => handle_run(&args[2..]),
        "replay" => handle_replay(&args[2..]),
        "snapshot" => handle_snapshot(&args[2..]),
        "validate" => handle_validate(&args[2..]),
        "serve" => handle_serve(&args[2..]),
        _ => {
            eprintln!(
                "Usage: market_tycoon_backend <run|replay|snapshot|validate|serve> [OPTIONS]"
            );
            process::exit(2);
        }
    };

    match result {
        Ok(()) => process::exit(0),
        Err(Error::InvalidScenario(msg)) | Err(Error::InvalidConfig(msg)) => {
            eprintln!("Invalid scenario/config: {msg}");
            process::exit(3);
        }
        Err(Error::ReplayMismatch(msg)) => {
            eprintln!("Replay mismatch: {msg}");
            process::exit(4);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(5);
        }
    }
}

fn parse_u64_arg(val: Option<&String>, flag: &str) -> Result<u64, Error> {
    val.ok_or_else(|| Error::InvalidCli(format!("{flag} requires a value")))?
        .parse::<u64>()
        .map_err(|_| Error::InvalidCli(format!("{flag} must be a positive integer")))
}

fn require_arg<'a>(val: Option<&'a String>, flag: &str) -> Result<&'a str, Error> {
    val.map(String::as_str)
        .ok_or_else(|| Error::InvalidCli(format!("{flag} requires a value")))
}

fn handle_run(args: &[String]) -> Result<(), Error> {
    let mut ticks: u64 = 100;
    let mut seed: u64 = 42;
    let mut scenario_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut replay_out: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--ticks" => {
                i += 1;
                ticks = parse_u64_arg(args.get(i), "--ticks")?;
            }
            "--seed" => {
                i += 1;
                seed = parse_u64_arg(args.get(i), "--seed")?;
            }
            "--scenario" => {
                i += 1;
                scenario_path = Some(PathBuf::from(require_arg(args.get(i), "--scenario")?));
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(require_arg(args.get(i), "--out")?));
            }
            "--replay-out" => {
                i += 1;
                replay_out = Some(PathBuf::from(require_arg(args.get(i), "--replay-out")?));
            }
            flag => return Err(Error::InvalidCli(format!("Unknown flag: {flag}"))),
        }
        i += 1;
    }

    let scenario_file =
        scenario_path.ok_or_else(|| Error::InvalidCli("--scenario is required".into()))?;
    let report_file = out_path.ok_or_else(|| Error::InvalidCli("--out is required".into()))?;

    let scen = scenario::scenario_loader::ScenarioLoader::load_from_file(&scenario_file)?;
    let config_obj = config::sim_config::SimConfig::from_scenario(&scen, seed, ticks);

    let mut engine = create_engine(&config_obj);
    let event_log = run_simulation(&mut engine, &config_obj);

    let sim_report = report::sim_report::SimReport::generate(&engine, &event_log, &config_obj);
    let report_json = io::json_codec::JsonCodec::encode(&sim_report)?;
    std::fs::write(&report_file, report_json).map_err(|e| Error::Io(e.to_string()))?;

    if let Some(rp) = replay_out {
        let rf = replay::replay_file::ReplayFile::new(seed, ticks, event_log.events().to_vec());
        let replay_json = replay::replay_codec::ReplayCodec::encode(&rf)?;
        std::fs::write(rp, replay_json).map_err(|e| Error::Io(e.to_string()))?;
    }

    Ok(())
}

fn handle_replay(args: &[String]) -> Result<(), Error> {
    let mut replay_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--replay" => {
                i += 1;
                replay_path = Some(PathBuf::from(require_arg(args.get(i), "--replay")?));
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(require_arg(args.get(i), "--out")?));
            }
            flag => return Err(Error::InvalidCli(format!("Unknown flag: {flag}"))),
        }
        i += 1;
    }

    let rp = replay_path.ok_or_else(|| Error::InvalidCli("--replay is required".into()))?;
    let op = out_path.ok_or_else(|| Error::InvalidCli("--out is required".into()))?;

    let data = std::fs::read_to_string(&rp).map_err(|e| Error::Io(e.to_string()))?;
    let rf = replay::replay_codec::ReplayCodec::decode(&data)?;
    let replayed_report = replay::replay_engine::ReplayEngine::replay(&rf)?;
    let report_json = io::json_codec::JsonCodec::encode(&replayed_report)?;
    std::fs::write(&op, report_json).map_err(|e| Error::Io(e.to_string()))?;

    Ok(())
}

fn handle_snapshot(args: &[String]) -> Result<(), Error> {
    let mut replay_path: Option<PathBuf> = None;
    let mut at_tick: u64 = 0;
    let mut out_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--replay" => {
                i += 1;
                replay_path = Some(PathBuf::from(require_arg(args.get(i), "--replay")?));
            }
            "--at-tick" => {
                i += 1;
                at_tick = parse_u64_arg(args.get(i), "--at-tick")?;
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(require_arg(args.get(i), "--out")?));
            }
            flag => return Err(Error::InvalidCli(format!("Unknown flag: {flag}"))),
        }
        i += 1;
    }

    let rp = replay_path.ok_or_else(|| Error::InvalidCli("--replay is required".into()))?;
    let op = out_path.ok_or_else(|| Error::InvalidCli("--out is required".into()))?;

    let data = std::fs::read_to_string(&rp).map_err(|e| Error::Io(e.to_string()))?;
    let rf = replay::replay_codec::ReplayCodec::decode(&data)?;
    let snap = replay::replay_engine::ReplayEngine::snapshot_at(&rf, at_tick)?;
    let snap_json = io::json_codec::JsonCodec::encode(&snap)?;
    std::fs::write(&op, snap_json).map_err(|e| Error::Io(e.to_string()))?;

    Ok(())
}

fn handle_validate(args: &[String]) -> Result<(), Error> {
    let mut scenario_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--scenario" => {
                i += 1;
                scenario_path = Some(PathBuf::from(require_arg(args.get(i), "--scenario")?));
            }
            flag => return Err(Error::InvalidCli(format!("Unknown flag: {flag}"))),
        }
        i += 1;
    }

    let sf = scenario_path.ok_or_else(|| Error::InvalidCli("--scenario is required".into()))?;
    let _scen = scenario::scenario_loader::ScenarioLoader::load_from_file(&sf)?;
    eprintln!("Scenario is valid.");
    Ok(())
}

fn handle_serve(args: &[String]) -> Result<(), Error> {
    let mut scenario_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--scenario" => {
                i += 1;
                scenario_path = Some(PathBuf::from(require_arg(args.get(i), "--scenario")?));
            }
            flag => return Err(Error::InvalidCli(format!("Unknown flag: {flag}"))),
        }
        i += 1;
    }

    let sf = scenario_path.ok_or_else(|| Error::InvalidCli("--scenario is required".into()))?;
    let scen = scenario::scenario_loader::ScenarioLoader::load_from_file(&sf)?;
    transport::ipc_server::IpcServer::run(scen)
}

pub(crate) struct SimEngine {
    pub(crate) companies:
        std::collections::BTreeMap<model::company_id::CompanyId, model::company::Company>,
    pub(crate) stores: std::collections::BTreeMap<model::store_id::StoreId, model::store::Store>,
    pub(crate) inventories:
        std::collections::BTreeMap<model::store_id::StoreId, model::inventory::Inventory>,
    pub(crate) ledger: finance::ledger::Ledger,
    pub(crate) clock: time::tick_clock::TickClock,
    pub(crate) rng_state: u64,
}

pub(crate) fn create_engine(config: &config::sim_config::SimConfig) -> SimEngine {
    let mut companies = std::collections::BTreeMap::new();
    let mut stores = std::collections::BTreeMap::new();
    let mut inventories = std::collections::BTreeMap::new();

    for (i, sc) in config.companies.iter().enumerate() {
        let cid = model::company_id::CompanyId(i as u64);
        let company = model::company::Company::new(cid, sc.name.clone(), sc.budget);
        companies.insert(cid, company);

        for (j, ss) in sc.stores.iter().enumerate() {
            let sid = model::store_id::StoreId(i as u64 * 1000 + j as u64);
            let store = model::store::Store::new(sid, cid, ss.name.clone());
            stores.insert(sid, store);
            inventories.insert(sid, model::inventory::Inventory::new(sid));
        }
    }

    SimEngine {
        companies,
        stores,
        inventories,
        ledger: finance::ledger::Ledger::new(),
        clock: time::tick_clock::TickClock::new(config.seed, config.ticks),
        rng_state: config.seed,
    }
}

pub(crate) fn run_simulation(
    engine: &mut SimEngine,
    config: &config::sim_config::SimConfig,
) -> events::event_log::EventLog {
    let mut event_log = events::event_log::EventLog::new();

    while !engine.clock.is_done() {
        let tick = engine.clock.current();

        // Supply phase
        for contract in &config.contracts {
            supply::supply_engine::SupplyEngine::process_delivery(
                contract,
                &tick,
                &mut engine.inventories,
                &mut engine.ledger,
                &mut event_log,
            );
        }

        // Pricing phase
        for (sid, inv) in &engine.inventories {
            let store = &engine.stores[sid];
            let company = &engine.companies[&store.owner()];
            pricing::pricing_engine::PricingEngine::update_prices(
                &tick,
                company,
                inv,
                &config.pricing_policy,
                &mut event_log,
            );
        }

        // Demand phase
        let rng_val = next_rng(&mut engine.rng_state);
        demand::demand_engine::DemandEngine::process_demand(
            &tick,
            &config.demand_model,
            &config.segments,
            rng_val,
            &mut engine.inventories,
            &mut engine.ledger,
            &mut event_log,
        );

        // Logistics phase
        logistics::logistics_engine::LogisticsEngine::process_shipments(
            &tick,
            &config.contracts,
            &mut engine.inventories,
            &mut event_log,
        );

        engine.clock.advance();
    }

    event_log
}

pub(crate) fn next_rng(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}
