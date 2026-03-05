// projects/products/unstable/colony_manager/backend/src/controller.rs
use crate::config::sim_config::SimConfig;
use crate::diagnostics::colony_manager_error::ColonyManagerError;
use crate::io::json_codec::JsonCodec;
use crate::replay::replay_codec::ReplayCodec;
use crate::replay::replay_engine::ReplayEngine;
use crate::replay::replay_file::ReplayFile;
use crate::rng::seed::Seed;
use crate::scenarios::scenario_loader::ScenarioLoader;
use crate::sim_engine::SimEngine;
use std::path::PathBuf;

pub struct Controller;

impl Controller {
    pub fn run(args: &[String]) -> Result<String, ColonyManagerError> {
        let mut config = SimConfig::default();
        let mut scenario_path: Option<PathBuf> = None;
        let mut out_path: Option<PathBuf> = None;
        let mut replay_out: Option<PathBuf> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--ticks" => {
                    i += 1;
                    config.max_ticks = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(100);
                }
                "--seed" => {
                    i += 1;
                    config.seed = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(42);
                }
                "--scenario" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        scenario_path = Some(PathBuf::from(v));
                    }
                }
                "--out" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        out_path = Some(PathBuf::from(v));
                    }
                }
                "--replay-out" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        replay_out = Some(PathBuf::from(v));
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let out = out_path.ok_or_else(|| ColonyManagerError::Sim("--out required".to_string()))?;
        let scenario = if let Some(path) = scenario_path {
            ScenarioLoader::load(&path)?
        } else {
            ScenarioLoader::default_scenario("hauling_basic")
        };

        let (report, rng_draws) = SimEngine::run(&scenario, config.max_ticks, config.seed)?;
        JsonCodec::save(&report, &out)?;

        if let Some(path) = replay_out {
            let replay = ReplayFile {
                seed: Seed(config.seed),
                ticks: config.max_ticks,
                scenario_name: scenario.name.clone(),
                rng_draws,
            };
            ReplayCodec::save(&replay, &path)?;
        }

        Ok(format!("Run complete. RunHash: {}", report.run_hash.0))
    }

    pub fn replay(args: &[String]) -> Result<String, ColonyManagerError> {
        let mut replay_path: Option<PathBuf> = None;
        let mut out_path: Option<PathBuf> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--replay" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        replay_path = Some(PathBuf::from(v));
                    }
                }
                "--out" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        out_path = Some(PathBuf::from(v));
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let replay =
            replay_path.ok_or_else(|| ColonyManagerError::Sim("--replay required".to_string()))?;
        let out = out_path.ok_or_else(|| ColonyManagerError::Sim("--out required".to_string()))?;

        let replay_file = ReplayCodec::load(&replay)?;
        let report = ReplayEngine::replay(&replay_file)?;
        JsonCodec::save(&report, &out)?;

        Ok(format!("Replay complete. RunHash: {}", report.run_hash.0))
    }
}
