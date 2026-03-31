mod assets;
mod config;
mod decision;
mod diagnostics;
mod fundamentals;
mod history;
mod journal;
mod market_data;
mod neural;
mod portfolio;
mod replay;
mod report;
mod risk;
mod scenario;
mod sentiment;
mod transport;

#[cfg(test)]
mod tests;

use crate::config::EngineConfig;
use crate::config::FeatureGateConfig;
use crate::diagnostics::EngineError;

fn main() -> Result<(), EngineError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "analyze-asset" => cmd_analyze_asset(&args),
        "analyze-portfolio" => cmd_analyze_portfolio(&args),
        "replay" => cmd_replay(&args),
        "journal" => cmd_journal(&args),
        "scenario" => cmd_scenario(&args),
        _ => {
            print_usage();
            Ok(())
        }
    }
}

fn print_usage() {
    eprintln!("Usage: investment_decision_engine_backend <command> [options]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  analyze-asset       Analyze a single asset");
    eprintln!("  analyze-portfolio   Analyze a full portfolio");
    eprintln!("  replay              Replay a previous decision");
    eprintln!("  journal             View decision journal");
    eprintln!("  scenario            Evaluate stress scenarios");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --asset <file>      Asset profile JSON file");
    eprintln!("  --market <file>     Market data JSON file");
    eprintln!("  --portfolio <file>  Portfolio state JSON file");
    eprintln!("  --replay <file>     Replay file");
    eprintln!("  --out <file>        Output file path");
}

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

fn cmd_analyze_asset(args: &[String]) -> Result<(), EngineError> {
    let asset_path = parse_arg(args, "--asset")
        .ok_or_else(|| EngineError::Input("--asset is required".to_string()))?;
    let market_path = parse_arg(args, "--market")
        .ok_or_else(|| EngineError::Input("--market is required".to_string()))?;
    let out_path = parse_arg(args, "--out");

    let asset_json = std::fs::read_to_string(&asset_path)
        .map_err(|e| EngineError::Io(format!("failed to read asset file: {e}")))?;
    let market_json = std::fs::read_to_string(&market_path)
        .map_err(|e| EngineError::Io(format!("failed to read market file: {e}")))?;

    let config = EngineConfig::default();
    let gate = FeatureGateConfig::from_config(&config);

    let asset_profile: assets::AssetProfile = common_json::from_str(&asset_json)
        .map_err(|e| EngineError::Parse(format!("invalid asset JSON: {e}")))?;
    let market_snapshot: market_data::MarketSnapshot = common_json::from_str(&market_json)
        .map_err(|e| EngineError::Parse(format!("invalid market JSON: {e}")))?;

    let report = report::AssetReport::generate(&asset_profile, &market_snapshot, &config, &gate);

    let output = common_json::to_json_string_pretty(&report)
        .map_err(|e| EngineError::Serialization(format!("{e}")))?;

    match out_path {
        Some(path) => {
            std::fs::write(&path, &output)
                .map_err(|e| EngineError::Io(format!("failed to write output: {e}")))?;
            eprintln!("Report written to {path}");
        }
        None => println!("{output}"),
    }

    Ok(())
}

fn cmd_analyze_portfolio(args: &[String]) -> Result<(), EngineError> {
    let portfolio_path = parse_arg(args, "--portfolio")
        .ok_or_else(|| EngineError::Input("--portfolio is required".to_string()))?;
    let market_path = parse_arg(args, "--market")
        .ok_or_else(|| EngineError::Input("--market is required".to_string()))?;
    let out_path = parse_arg(args, "--out");

    let portfolio_json = std::fs::read_to_string(&portfolio_path)
        .map_err(|e| EngineError::Io(format!("failed to read portfolio file: {e}")))?;
    let market_json = std::fs::read_to_string(&market_path)
        .map_err(|e| EngineError::Io(format!("failed to read market file: {e}")))?;

    let config = EngineConfig::default();
    let gate = FeatureGateConfig::from_config(&config);

    let portfolio_state: portfolio::PortfolioState = common_json::from_str(&portfolio_json)
        .map_err(|e| EngineError::Parse(format!("invalid portfolio JSON: {e}")))?;
    let market_snapshot: market_data::MarketSnapshot = common_json::from_str(&market_json)
        .map_err(|e| EngineError::Parse(format!("invalid market JSON: {e}")))?;

    let report =
        report::PortfolioReport::generate(&portfolio_state, &market_snapshot, &config, &gate);

    let output = common_json::to_json_string_pretty(&report)
        .map_err(|e| EngineError::Serialization(format!("{e}")))?;

    match out_path {
        Some(path) => {
            std::fs::write(&path, &output)
                .map_err(|e| EngineError::Io(format!("failed to write output: {e}")))?;
            eprintln!("Report written to {path}");
        }
        None => println!("{output}"),
    }

    Ok(())
}

fn cmd_replay(args: &[String]) -> Result<(), EngineError> {
    let replay_path = parse_arg(args, "--replay")
        .ok_or_else(|| EngineError::Input("--replay is required".to_string()))?;
    let out_path = parse_arg(args, "--out");

    let replay_json = std::fs::read_to_string(&replay_path)
        .map_err(|e| EngineError::Io(format!("failed to read replay file: {e}")))?;

    let replay_file: replay::ReplayFile = common_json::from_str(&replay_json)
        .map_err(|e| EngineError::Parse(format!("invalid replay JSON: {e}")))?;

    let config = EngineConfig::default();
    let gate = FeatureGateConfig::from_config(&config);
    let result = replay::ReplayEngine::execute(&replay_file, &config, &gate);

    let output = common_json::to_json_string_pretty(&result)
        .map_err(|e| EngineError::Serialization(format!("{e}")))?;

    match out_path {
        Some(path) => {
            std::fs::write(&path, &output)
                .map_err(|e| EngineError::Io(format!("failed to write output: {e}")))?;
            eprintln!("Replay written to {path}");
        }
        None => println!("{output}"),
    }

    Ok(())
}

fn cmd_journal(args: &[String]) -> Result<(), EngineError> {
    let journal_path = parse_arg(args, "--journal")
        .ok_or_else(|| EngineError::Input("--journal is required".to_string()))?;
    let out_path = parse_arg(args, "--out");

    let journal_json = std::fs::read_to_string(&journal_path)
        .map_err(|e| EngineError::Io(format!("failed to read journal file: {e}")))?;

    let entries: Vec<journal::DecisionEntry> = common_json::from_str(&journal_json)
        .map_err(|e| EngineError::Parse(format!("invalid journal JSON: {e}")))?;

    let output = common_json::to_json_string_pretty(&entries)
        .map_err(|e| EngineError::Serialization(format!("{e}")))?;

    match out_path {
        Some(path) => {
            std::fs::write(&path, &output)
                .map_err(|e| EngineError::Io(format!("failed to write output: {e}")))?;
            eprintln!("Journal written to {path}");
        }
        None => println!("{output}"),
    }

    Ok(())
}

fn cmd_scenario(args: &[String]) -> Result<(), EngineError> {
    let scenario_path = parse_arg(args, "--scenario")
        .ok_or_else(|| EngineError::Input("--scenario is required".to_string()))?;
    let asset_path = parse_arg(args, "--asset")
        .ok_or_else(|| EngineError::Input("--asset is required".to_string()))?;
    let out_path = parse_arg(args, "--out");

    let scenario_json = std::fs::read_to_string(&scenario_path)
        .map_err(|e| EngineError::Io(format!("failed to read scenario file: {e}")))?;
    let asset_json = std::fs::read_to_string(&asset_path)
        .map_err(|e| EngineError::Io(format!("failed to read asset file: {e}")))?;

    let scenario: scenario::Scenario = common_json::from_str(&scenario_json)
        .map_err(|e| EngineError::Parse(format!("invalid scenario JSON: {e}")))?;
    let asset_profile: assets::AssetProfile = common_json::from_str(&asset_json)
        .map_err(|e| EngineError::Parse(format!("invalid asset JSON: {e}")))?;

    let config = EngineConfig::default();
    let result = scenario::ScenarioEngine::evaluate(&scenario, &asset_profile, &config);

    let output = common_json::to_json_string_pretty(&result)
        .map_err(|e| EngineError::Serialization(format!("{e}")))?;

    match out_path {
        Some(path) => {
            std::fs::write(&path, &output)
                .map_err(|e| EngineError::Io(format!("failed to write output: {e}")))?;
            eprintln!("Scenario result written to {path}");
        }
        None => println!("{output}"),
    }

    Ok(())
}
