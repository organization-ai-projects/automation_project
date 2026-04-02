mod agents;
mod config;
mod diagnostics;
mod events;
mod io;
mod market;
mod policy;
mod replay;
mod report;
mod sim;
mod snapshot;
mod time;

#[cfg(test)]
mod tests;

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("");

    let result = match command {
        "run" => handle_run(&args[2..]),
        "replay" => handle_replay(&args[2..]),
        _ => {
            eprintln!("Usage: econ_sandbox <run|replay> [OPTIONS]");
            eprintln!("  run    --ticks N --seed S [--json]");
            eprintln!("  replay <replay_file> [--json]");
            process::exit(2);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn handle_run(args: &[String]) -> Result<(), diagnostics::error::Error> {
    let mut ticks: u64 = 100;
    let mut seed: u64 = 42;
    let mut json_output = false;

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
            "--json" => {
                json_output = true;
            }
            _ => {}
        }
        i += 1;
    }

    let cfg = config::sim_config::SimConfig { seed, ticks };
    let (report, replay_data) = sim::sim_engine::SimEngine::run_sim(&cfg)?;

    if json_output {
        let json = io::json_codec::JsonCodec::encode(&report)?;
        println!("{json}");
    } else {
        println!("Run complete. Hash: {}", report.run_hash);
        println!(
            "  seed={}, ticks={}, events={}",
            report.seed, report.ticks, report.event_count
        );
    }

    let replay_json = replay::replay_codec::ReplayCodec::encode(&replay_data)?;
    let replay_path = format!("econ_sandbox_replay_s{}_t{}.json", seed, ticks);
    std::fs::write(&replay_path, &replay_json)
        .map_err(|e| diagnostics::error::Error::Io(e.to_string()))?;
    eprintln!("Replay file written to {replay_path}");

    Ok(())
}

fn handle_replay(args: &[String]) -> Result<(), diagnostics::error::Error> {
    if args.is_empty() {
        return Err(diagnostics::error::Error::InvalidCli(
            "replay requires <replay_file>".into(),
        ));
    }

    let replay_path = &args[0];
    let json_output = args.iter().any(|a| a == "--json");

    let data = std::fs::read_to_string(replay_path)
        .map_err(|e| diagnostics::error::Error::Io(e.to_string()))?;
    let replay_file = replay::replay_codec::ReplayCodec::decode(&data)?;
    let report = replay::replay_engine::ReplayEngine::replay(&replay_file)?;

    if json_output {
        let json = io::json_codec::JsonCodec::encode(&report)?;
        println!("{json}");
    } else {
        println!("Replay complete. Hash: {}", report.run_hash);
        println!(
            "  seed={}, ticks={}, events={}",
            report.seed, report.ticks, report.event_count
        );
    }

    Ok(())
}

fn parse_u64_arg(val: Option<&String>, flag: &str) -> Result<u64, diagnostics::error::Error> {
    val.ok_or_else(|| diagnostics::error::Error::InvalidCli(format!("{flag} requires a value")))?
        .parse::<u64>()
        .map_err(|_| {
            diagnostics::error::Error::InvalidCli(format!("{flag} must be a positive integer"))
        })
}
