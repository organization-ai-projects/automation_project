use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Request {
    Run {
        ticks: u64,
        seed: u64,
        scenario: PathBuf,
        out: PathBuf,
        replay_out: Option<PathBuf>,
    },
    Replay {
        replay: PathBuf,
        out: PathBuf,
    },
    Snapshot {
        replay: PathBuf,
        at_tick: u64,
        out: PathBuf,
    },
    Validate {
        scenario: PathBuf,
    },
}

impl Request {
    pub fn summary(&self) -> String {
        match self {
            Self::Run { .. } => "run".to_string(),
            Self::Replay { .. } => "replay".to_string(),
            Self::Snapshot { .. } => "snapshot".to_string(),
            Self::Validate { .. } => "validate".to_string(),
        }
    }

    pub fn as_args(&self) -> Vec<String> {
        match self {
            Self::Run {
                ticks,
                seed,
                scenario,
                out,
                replay_out,
            } => {
                let mut args = vec![
                    "run".to_string(),
                    "--ticks".to_string(),
                    ticks.to_string(),
                    "--seed".to_string(),
                    seed.to_string(),
                    "--scenario".to_string(),
                    scenario.display().to_string(),
                    "--out".to_string(),
                    out.display().to_string(),
                ];
                if let Some(path) = replay_out {
                    args.push("--replay-out".to_string());
                    args.push(path.display().to_string());
                }
                args
            }
            Self::Replay { replay, out } => vec![
                "replay".to_string(),
                "--replay".to_string(),
                replay.display().to_string(),
                "--out".to_string(),
                out.display().to_string(),
            ],
            Self::Snapshot {
                replay,
                at_tick,
                out,
            } => vec![
                "snapshot".to_string(),
                "--replay".to_string(),
                replay.display().to_string(),
                "--at-tick".to_string(),
                at_tick.to_string(),
                "--out".to_string(),
                out.display().to_string(),
            ],
            Self::Validate { scenario } => vec![
                "validate".to_string(),
                "--scenario".to_string(),
                scenario.display().to_string(),
            ],
        }
    }
}
