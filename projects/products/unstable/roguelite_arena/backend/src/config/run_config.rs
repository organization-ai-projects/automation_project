use crate::diagnostics::RogueliteArenaError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RunConfig {
    pub(crate) ticks: u64,
    pub(crate) seed: u64,
    pub(crate) scenario_path: Option<PathBuf>,
    pub(crate) out_path: Option<PathBuf>,
    pub(crate) replay_out_path: Option<PathBuf>,
}

impl RunConfig {
    pub(crate) fn parse(args: &[String]) -> Result<Self, RogueliteArenaError> {
        let mut ticks: Option<u64> = None;
        let mut seed: Option<u64> = None;
        let mut scenario_path: Option<PathBuf> = None;
        let mut out_path: Option<PathBuf> = None;
        let mut replay_out_path: Option<PathBuf> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--ticks" => {
                    i += 1;
                    let val = args
                        .get(i)
                        .ok_or_else(|| {
                            RogueliteArenaError::InvalidConfig("missing --ticks value".to_string())
                        })?
                        .parse::<u64>()
                        .map_err(|e| {
                            RogueliteArenaError::InvalidConfig(format!("invalid --ticks: {e}"))
                        })?;
                    ticks = Some(val);
                }
                "--seed" => {
                    i += 1;
                    let val = args
                        .get(i)
                        .ok_or_else(|| {
                            RogueliteArenaError::InvalidConfig("missing --seed value".to_string())
                        })?
                        .parse::<u64>()
                        .map_err(|e| {
                            RogueliteArenaError::InvalidConfig(format!("invalid --seed: {e}"))
                        })?;
                    seed = Some(val);
                }
                "--scenario" => {
                    i += 1;
                    scenario_path = Some(PathBuf::from(args.get(i).ok_or_else(|| {
                        RogueliteArenaError::InvalidConfig("missing --scenario value".to_string())
                    })?));
                }
                "--out" => {
                    i += 1;
                    out_path = Some(PathBuf::from(args.get(i).ok_or_else(|| {
                        RogueliteArenaError::InvalidConfig("missing --out value".to_string())
                    })?));
                }
                "--replay-out" => {
                    i += 1;
                    replay_out_path = Some(PathBuf::from(args.get(i).ok_or_else(|| {
                        RogueliteArenaError::InvalidConfig("missing --replay-out value".to_string())
                    })?));
                }
                _ => {}
            }
            i += 1;
        }

        Ok(Self {
            ticks: ticks.unwrap_or(100),
            seed: seed.unwrap_or(42),
            scenario_path,
            out_path,
            replay_out_path,
        })
    }
}
