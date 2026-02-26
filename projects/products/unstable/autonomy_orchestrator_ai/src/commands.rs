// projects/products/unstable/autonomy_orchestrator_ai/src/commands.rs
use clap::Subcommand;

use crate::configs::{ConfigCanonicalizeArgs, ConfigValidateArgs};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Fixture {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    LinkedStack {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    ConfigValidate(ConfigValidateArgs),
    ConfigCanonicalize(ConfigCanonicalizeArgs),
}
