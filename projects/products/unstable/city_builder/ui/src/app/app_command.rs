use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum AppCommand {
    Run {
        #[arg(long)]
        ticks: u64,
        #[arg(long)]
        seed: u64,
        #[arg(long)]
        scenario: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        replay_out: Option<PathBuf>,
    },
    Replay {
        #[arg(long)]
        replay: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    Snapshot {
        #[arg(long)]
        replay: PathBuf,
        #[arg(long)]
        at_tick: u64,
        #[arg(long)]
        out: PathBuf,
    },
    Validate {
        #[arg(long)]
        scenario: PathBuf,
    },
}
