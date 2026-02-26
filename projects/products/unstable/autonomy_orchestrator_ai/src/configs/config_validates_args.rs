// projects/products/unstable/autonomy_orchestrator_ai/src/configs/config_validate_args.rs
use std::path::PathBuf;

use clap::Args;

#[derive(Args, Debug)]
pub struct ConfigValidateArgs {
    pub config_path: PathBuf,
    #[arg(long)]
    pub ai_config_only_binary: bool,
}
