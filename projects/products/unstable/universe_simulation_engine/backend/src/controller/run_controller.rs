use crate::config::physics_config::PhysicsConfig;
use crate::config::sim_config::SimConfig;
use crate::diagnostics::engine_error::EngineError;
use crate::io::binary_codec;
use crate::io::ron_codec;
use crate::sim::sim_engine::SimEngine;
use std::path::PathBuf;

pub struct RunController;

impl RunController {
    pub fn run(args: &[String]) -> Result<String, EngineError> {
        let mut config = SimConfig::default();
        let mut out_path: Option<PathBuf> = None;
        let mut save_bin: Option<PathBuf> = None;
        let mut save_ron: Option<PathBuf> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--ticks" => {
                    i += 1;
                    config.max_ticks = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(1000);
                }
                "--seed" => {
                    i += 1;
                    config.seed = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(42);
                }
                "--ticks-per-era" => {
                    i += 1;
                    config.ticks_per_era = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(50);
                }
                "--out" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        out_path = Some(PathBuf::from(v));
                    }
                }
                "--save-bin" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        save_bin = Some(PathBuf::from(v));
                    }
                }
                "--save-ron" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        save_ron = Some(PathBuf::from(v));
                    }
                }
                "--no-gravity" => config.physics.gravity_enabled = false,
                "--no-electromagnetism" => config.physics.electromagnetism_enabled = false,
                "--no-strong-nuclear" => config.physics.strong_nuclear_enabled = false,
                "--no-weak-nuclear" => config.physics.weak_nuclear_enabled = false,
                "--no-dark-matter" => config.physics.dark_matter_enabled = false,
                "--no-dark-energy" => config.physics.dark_energy_enabled = false,
                "--no-thermodynamics" => config.physics.thermodynamics_enabled = false,
                "--config" => {
                    i += 1;
                    if let Some(path) = args.get(i) {
                        let loaded: PhysicsConfig =
                            ron_codec::load_ron(std::path::Path::new(path))?;
                        config.physics = loaded;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let out = out_path.ok_or_else(|| EngineError::Sim("--out required".to_string()))?;

        let report = SimEngine::run(&config)?;

        if let Some(bin_path) = save_bin {
            binary_codec::save_binary(&report, &bin_path)?;
        }
        if let Some(ron_path) = save_ron {
            ron_codec::save_ron(&report, &ron_path)?;
        }

        let json =
            common_json::to_json_string(&report).map_err(|e| EngineError::Json(e.to_string()))?;
        std::fs::write(&out, json.as_bytes())?;

        Ok(format!("Run complete. RunHash: {}", report.run_hash.0))
    }
}
