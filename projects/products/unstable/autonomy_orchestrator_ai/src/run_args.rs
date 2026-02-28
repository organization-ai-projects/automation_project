// projects/products/unstable/autonomy_orchestrator_ai/src/run_args.rs
use std::path::PathBuf;

use clap::{ArgAction, Args};

use crate::{
    cli_command::{CliCiStatus, CliPolicyStatus, CliReviewStatus, CliRiskTier},
    cli_value_parsers::{
        parse_decision_contribution_cli, parse_decision_reliability_input_cli, parse_env_pair_cli,
        parse_reviewer_verdict_cli,
    },
    domain::{DecisionContribution, DecisionReliabilityInput, ReviewerVerdict},
};

#[derive(Args, Debug, Clone)]
pub struct RunArgs {
    #[arg(default_value = "./out")]
    pub output_dir: PathBuf,

    #[arg(long)]
    pub simulate_blocked: bool,
    #[arg(long)]
    pub verbose: bool,
    #[arg(long)]
    pub resume: bool,
    #[arg(long, default_value_t = 30_000)]
    pub timeout_ms: u64,

    #[arg(long, value_enum, default_value_t = CliPolicyStatus::Unknown)]
    pub policy_status: CliPolicyStatus,
    #[arg(long, value_enum, default_value_t = CliCiStatus::Missing)]
    pub ci_status: CliCiStatus,
    #[arg(long, value_enum, default_value_t = CliReviewStatus::Missing)]
    pub review_status: CliReviewStatus,
    #[arg(long, default_value_t = 70)]
    pub decision_threshold: u8,
    #[arg(long)]
    pub decision_require_contributions: bool,
    #[arg(long = "decision-contribution", value_parser = parse_decision_contribution_cli, action = ArgAction::Append)]
    pub decision_contributions: Vec<DecisionContribution>,
    #[arg(long = "decision-reliability", value_parser = parse_decision_reliability_input_cli, action = ArgAction::Append)]
    pub decision_reliability_inputs: Vec<DecisionReliabilityInput>,
    #[arg(long = "reviewer-verdict", value_parser = parse_reviewer_verdict_cli, action = ArgAction::Append)]
    pub reviewer_verdicts: Vec<ReviewerVerdict>,

    #[arg(long)]
    pub checkpoint_path: Option<PathBuf>,
    #[arg(long)]
    pub cycle_memory_path: Option<PathBuf>,
    #[arg(long)]
    pub next_actions_path: Option<PathBuf>,
    #[arg(long)]
    pub autonomous_loop: bool,
    #[arg(long, default_value_t = 3)]
    pub autonomous_max_runs: u32,
    #[arg(long, default_value_t = 300_000)]
    pub autonomous_max_duration_ms: u64,
    #[arg(long, default_value_t = 2)]
    pub autonomous_same_error_limit: u32,

    #[arg(long)]
    pub manager_bin: Option<String>,
    #[arg(long = "manager-arg", action = ArgAction::Append)]
    pub manager_args: Vec<String>,
    #[arg(long = "manager-env", value_parser = parse_env_pair_cli, action = ArgAction::Append)]
    pub manager_env: Vec<(String, String)>,
    #[arg(long = "manager-expected-artifact", action = ArgAction::Append)]
    pub manager_expected_artifacts: Vec<String>,

    #[arg(long)]
    pub executor_bin: Option<String>,
    #[arg(long = "executor-arg", action = ArgAction::Append)]
    pub executor_args: Vec<String>,
    #[arg(long = "executor-env", value_parser = parse_env_pair_cli, action = ArgAction::Append)]
    pub executor_env: Vec<(String, String)>,
    #[arg(long = "executor-expected-artifact", action = ArgAction::Append)]
    pub executor_expected_artifacts: Vec<String>,

    #[arg(long, default_value_t = 1)]
    pub execution_max_iterations: u32,
    #[arg(long, default_value_t = 0)]
    pub reviewer_remediation_max_cycles: u32,

    #[arg(long)]
    pub reviewer_bin: Option<String>,
    #[arg(long = "reviewer-arg", action = ArgAction::Append)]
    pub reviewer_args: Vec<String>,
    #[arg(long = "reviewer-env", value_parser = parse_env_pair_cli, action = ArgAction::Append)]
    pub reviewer_env: Vec<(String, String)>,
    #[arg(long = "reviewer-expected-artifact", action = ArgAction::Append)]
    pub reviewer_expected_artifacts: Vec<String>,

    // Parsed manually from raw argv to preserve "binds to last --validation-bin" semantics.
    #[arg(long = "validation-bin", action = ArgAction::Append)]
    pub _validation_bins: Vec<String>,
    #[arg(long = "validation-arg", action = ArgAction::Append)]
    pub _validation_args: Vec<String>,
    #[arg(long = "validation-env", value_parser = parse_env_pair_cli, action = ArgAction::Append)]
    pub _validation_env: Vec<(String, String)>,

    #[arg(long)]
    pub validation_from_planning_context: bool,

    #[arg(long, default_value = ".")]
    pub repo_root: PathBuf,
    #[arg(long)]
    pub planning_context_artifact: Option<PathBuf>,

    #[arg(long)]
    pub delivery_enabled: bool,
    #[arg(long)]
    pub delivery_dry_run: bool,
    #[arg(long)]
    pub delivery_branch: Option<String>,
    #[arg(long)]
    pub delivery_commit_message: Option<String>,
    #[arg(long)]
    pub delivery_pr_enabled: bool,
    #[arg(long)]
    pub delivery_pr_number: Option<String>,
    #[arg(long)]
    pub delivery_pr_base: Option<String>,
    #[arg(long)]
    pub delivery_pr_title: Option<String>,
    #[arg(long)]
    pub delivery_pr_body: Option<String>,

    #[arg(long)]
    pub config_save_ron: Option<PathBuf>,
    #[arg(long)]
    pub config_save_bin: Option<PathBuf>,
    #[arg(long)]
    pub config_save_json: Option<PathBuf>,
    #[arg(long = "config-save")]
    pub config_save_auto: Option<PathBuf>,

    #[arg(long)]
    pub config_load_ron: Option<PathBuf>,
    #[arg(long)]
    pub config_load_bin: Option<PathBuf>,
    #[arg(long)]
    pub config_load_json: Option<PathBuf>,
    #[arg(long = "config-load")]
    pub config_load_auto: Option<PathBuf>,

    #[arg(long)]
    pub ai_config_only_binary: bool,

    #[arg(long)]
    pub autofix_enabled: bool,
    #[arg(long)]
    pub autofix_bin: Option<String>,
    #[arg(long = "autofix-arg", action = ArgAction::Append)]
    pub autofix_args: Vec<String>,
    #[arg(long, default_value_t = 3)]
    pub autofix_max_attempts: u32,
    pub hard_gates_file: Option<PathBuf>,
    #[arg(long, default_value_t = 3)]
    pub planner_fallback_max_steps: u32,
    #[arg(long, value_enum)]
    pub risk_tier_override: Option<CliRiskTier>,
    #[arg(long, default_value_t = false)]
    pub risk_allow_high: bool,
}
