// projects/products/unstable/autonomy_orchestrator_ai/src/cli_command/cli_policy_status.rs
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliPolicyStatus {
    Allow,
    Deny,
    Unknown,
}
