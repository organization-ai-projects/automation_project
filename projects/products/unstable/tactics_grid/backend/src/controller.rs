use crate::diagnostics::tactics_grid_error::TacticsGridError;
use crate::io::json_codec::JsonCodec;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::rng::seed::Seed;
use crate::scenario::scenario_loader::ScenarioLoader;
use crate::turn::turn_engine::TurnEngine;
use std::path::Path;

pub struct Controller;

impl Controller {
    pub fn run(args: &[String]) -> Result<String, TacticsGridError> {
        let mut seed: u64 = 42;
        let mut scenario_source: Option<String> = None;
        let mut out_path: Option<String> = None;
        let mut replay_out: Option<String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--seed" => {
                    i += 1;
                    seed = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(42);
                }
                "--scenario" => {
                    i += 1;
                    scenario_source = args.get(i).cloned();
                }
                "--out" => {
                    i += 1;
                    out_path = args.get(i).cloned();
                }
                "--replay-out" => {
                    i += 1;
                    replay_out = args.get(i).cloned();
                }
                _ => {}
            }
            i += 1;
        }

        let out = out_path
            .ok_or_else(|| TacticsGridError::InvalidScenario("--out is required".to_string()))?;

        let scenario = match &scenario_source {
            Some(src) => {
                let path = Path::new(src);
                if path.exists() {
                    ScenarioLoader::load_from_file(path)?
                } else {
                    ScenarioLoader::default_scenario(src)?
                }
            }
            None => ScenarioLoader::default_scenario("skirmish")?,
        };

        let (report, draws) = TurnEngine::run_battle(&scenario, Seed(seed))?;
        JsonCodec::save_report(&report, Path::new(&out))?;

        if let Some(replay_path) = replay_out {
            let replay = ReplayFile {
                seed: Seed(seed),
                scenario,
                rng_draws: draws,
            };
            ReplayCodec::save(&replay, Path::new(&replay_path))?;
        }

        Ok(format!(
            "Battle complete: {} turns, winner: {}, hash: {}",
            report.turns_played,
            report.winner.as_deref().unwrap_or("none"),
            report.run_hash.0,
        ))
    }

    pub fn replay(args: &[String]) -> Result<String, TacticsGridError> {
        let mut replay_path: Option<String> = None;
        let mut out_path: Option<String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--replay" => {
                    i += 1;
                    replay_path = args.get(i).cloned();
                }
                "--out" => {
                    i += 1;
                    out_path = args.get(i).cloned();
                }
                _ => {}
            }
            i += 1;
        }

        let replay_file = replay_path
            .ok_or_else(|| TacticsGridError::InvalidScenario("--replay is required".to_string()))?;
        let out = out_path
            .ok_or_else(|| TacticsGridError::InvalidScenario("--out is required".to_string()))?;

        let replay = ReplayCodec::load(Path::new(&replay_file))?;
        let report = ReplayEngine::replay(&replay)?;
        JsonCodec::save_report(&report, Path::new(&out))?;

        Ok(format!(
            "Replay complete: {} turns, winner: {}, hash: {}",
            report.turns_played,
            report.winner.as_deref().unwrap_or("none"),
            report.run_hash.0,
        ))
    }
}
