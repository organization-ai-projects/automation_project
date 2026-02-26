// projects/products/unstable/autonomy_orchestrator_ai/src/configs/config_canonicalize_args.rs
use std::path::PathBuf;

use clap::Args;

#[derive(Args, Debug)]
pub struct ConfigCanonicalizeArgs {
    pub input_config: PathBuf,
    pub output_bin_config: PathBuf,
    #[arg(long)]
    pub ai_config_only_binary: bool,
}
