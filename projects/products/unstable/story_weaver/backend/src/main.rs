mod config;
mod diagnostics;
mod dsl;
mod engine;
mod events;
mod export;
mod io;
mod replay;
mod report;
mod rng;
mod snapshot;
mod state;

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
            eprintln!("Usage: story_weaver_backend <run|replay> [OPTIONS]");
            eprintln!("  run    --seed S --script <rules_file> [--json]");
            eprintln!("  replay <replay_file> [--export markdown]");
            process::exit(2);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn handle_run(args: &[String]) -> Result<(), diagnostics::Error> {
    let mut seed: u64 = 42;
    let mut script_path: Option<String> = None;
    let mut json_output = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                i += 1;
                seed = parse_u64_arg(args.get(i), "--seed")?;
            }
            "--script" => {
                i += 1;
                script_path = Some(
                    args.get(i)
                        .ok_or_else(|| {
                            diagnostics::Error::InvalidCli("--script requires a path".into())
                        })?
                        .clone(),
                );
            }
            "--json" => {
                json_output = true;
            }
            _ => {}
        }
        i += 1;
    }

    let script_path = script_path.ok_or_else(|| {
        diagnostics::Error::InvalidCli("--script <rules_file> is required".into())
    })?;

    let script_data =
        std::fs::read_to_string(&script_path).map_err(|e| diagnostics::Error::Io(e.to_string()))?;
    let script = dsl::ScriptParser::parse(&script_data)?;

    let cfg = config::StoryConfig {
        seed,
        max_steps: script.max_steps,
    };
    let (report, replay_data) = engine::NarrativeEngine::run(&script, &cfg)?;

    if json_output {
        let json = export::JsonExporter::export(&report, replay_data.events.as_slice())?;
        println!("{json}");
    } else {
        println!("Run complete. Hash: {}", report.run_hash);
        println!(
            "  seed={}, steps={}, events={}",
            report.seed, report.steps_taken, report.event_count
        );
    }

    let replay_json = replay::ReplayCodec::encode(&replay_data)?;
    let replay_path = format!("story_weaver_replay_s{}.json", seed);
    std::fs::write(&replay_path, &replay_json)
        .map_err(|e| diagnostics::Error::Io(e.to_string()))?;
    eprintln!("Replay file written to {replay_path}");

    Ok(())
}

fn handle_replay(args: &[String]) -> Result<(), diagnostics::Error> {
    if args.is_empty() {
        return Err(diagnostics::Error::InvalidCli(
            "replay requires <replay_file>".into(),
        ));
    }

    let replay_path = &args[0];
    let export_markdown = args.iter().any(|a| a == "markdown");

    let data =
        std::fs::read_to_string(replay_path).map_err(|e| diagnostics::Error::Io(e.to_string()))?;
    let replay_file = replay::ReplayCodec::decode(&data)?;
    let report = replay::ReplayEngine::replay(&replay_file)?;

    if export_markdown {
        let md = export::MarkdownExporter::export(&report, &replay_file.events);
        println!("{md}");
    } else {
        let json = export::JsonExporter::export(&report, &replay_file.events)?;
        println!("{json}");
    }

    Ok(())
}

fn parse_u64_arg(val: Option<&String>, flag: &str) -> Result<u64, diagnostics::Error> {
    val.ok_or_else(|| diagnostics::Error::InvalidCli(format!("{flag} requires a value")))?
        .parse::<u64>()
        .map_err(|_| diagnostics::Error::InvalidCli(format!("{flag} must be a positive integer")))
}
