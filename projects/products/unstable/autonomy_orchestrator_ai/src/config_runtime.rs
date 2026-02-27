// projects/products/unstable/autonomy_orchestrator_ai/src/config_runtime.rs
use crate::configs::{
    ConfigCanonicalizeArgs, ConfigIoPlan, ConfigLoadMode, ConfigSaveMode, ConfigValidateArgs,
};
use crate::domain::OrchestratorConfig;
use crate::run_args::RunArgs;
use std::path::Path;
use std::process;

pub fn run_config_validate(args: ConfigValidateArgs) -> ! {
    if args.ai_config_only_binary && !is_binary_config_path(&args.config_path) {
        eprintln!(
            "AI binary-only mode forbids non-binary config path '{}'. Use .bin or no extension.",
            args.config_path.display()
        );
        process::exit(2);
    }

    let config = match OrchestratorConfig::load_auto(&args.config_path) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    let diagnostics = validate_orchestrator_config(&config);
    if diagnostics.is_empty() {
        println!("Config validation: OK ({})", args.config_path.display());
        process::exit(0);
    }

    eprintln!(
        "Config validation failed for '{}':",
        args.config_path.display()
    );
    for diag in diagnostics {
        eprintln!("- {diag}");
    }
    process::exit(3);
}

pub fn run_config_canonicalize(args: ConfigCanonicalizeArgs) -> ! {
    if args.ai_config_only_binary
        && (!is_binary_config_path(&args.input_config)
            || !is_binary_config_path(&args.output_bin_config))
    {
        eprintln!(
            "AI binary-only mode forbids non-binary config path(s). Use .bin or no extension."
        );
        process::exit(2);
    }

    let config = match OrchestratorConfig::load_auto(&args.input_config) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    let diagnostics = validate_orchestrator_config(&config);
    if !diagnostics.is_empty() {
        eprintln!(
            "Config canonicalization blocked, input config is invalid '{}':",
            args.input_config.display()
        );
        for diag in diagnostics {
            eprintln!("- {diag}");
        }
        process::exit(3);
    }

    if let Err(err) = config.save_bin(&args.output_bin_config) {
        eprintln!("{err}");
        process::exit(1);
    }

    println!(
        "Canonical binary config written: {} -> {}",
        args.input_config.display(),
        args.output_bin_config.display()
    );
    process::exit(0);
}

pub fn derive_config_io_plan(args: &RunArgs) -> Result<ConfigIoPlan, String> {
    let mut plan = ConfigIoPlan::default();

    let mut load_modes = Vec::new();
    if let Some(path) = &args.config_load_auto {
        load_modes.push(ConfigLoadMode::Auto(path.clone()));
    }
    if let Some(path) = &args.config_load_ron {
        load_modes.push(ConfigLoadMode::Ron(path.clone()));
    }
    if let Some(path) = &args.config_load_bin {
        load_modes.push(ConfigLoadMode::Bin(path.clone()));
    }
    if let Some(path) = &args.config_load_json {
        load_modes.push(ConfigLoadMode::Json(path.clone()));
    }
    if load_modes.len() > 1 {
        return Err(
            "Only one config load mode is allowed: choose exactly one of --config-load, --config-load-ron, --config-load-bin, --config-load-json"
                .to_string(),
        );
    }
    plan.load = load_modes.into_iter().next();

    if args.config_save_auto.is_some()
        && (args.config_save_ron.is_some()
            || args.config_save_bin.is_some()
            || args.config_save_json.is_some())
    {
        return Err(
            "When --config-save is used, do not combine it with --config-save-ron/--config-save-bin/--config-save-json"
                .to_string(),
        );
    }

    if let Some(path) = &args.config_save_auto {
        plan.saves.push(ConfigSaveMode::Auto(path.clone()));
    }
    if let Some(path) = &args.config_save_ron {
        plan.saves.push(ConfigSaveMode::Ron(path.clone()));
    }
    if let Some(path) = &args.config_save_bin {
        plan.saves.push(ConfigSaveMode::Bin(path.clone()));
    }
    if let Some(path) = &args.config_save_json {
        plan.saves.push(ConfigSaveMode::Json(path.clone()));
    }

    Ok(plan)
}

pub fn first_non_binary_config_path(plan: &ConfigIoPlan) -> Option<&Path> {
    if let Some(load) = &plan.load {
        let path = match load {
            ConfigLoadMode::Auto(path)
            | ConfigLoadMode::Ron(path)
            | ConfigLoadMode::Bin(path)
            | ConfigLoadMode::Json(path) => path,
        };
        if !is_binary_config_path(path) {
            return Some(path);
        }
    }

    for save in &plan.saves {
        let path = match save {
            ConfigSaveMode::Auto(path)
            | ConfigSaveMode::Ron(path)
            | ConfigSaveMode::Bin(path)
            | ConfigSaveMode::Json(path) => path,
        };
        if !is_binary_config_path(path) {
            return Some(path);
        }
    }

    None
}

pub fn load_config_by_mode(mode: &ConfigLoadMode) -> Result<OrchestratorConfig, String> {
    match mode {
        ConfigLoadMode::Auto(path) => OrchestratorConfig::load_auto(path),
        ConfigLoadMode::Ron(path) => OrchestratorConfig::load_ron(path),
        ConfigLoadMode::Bin(path) => OrchestratorConfig::load_bin(path),
        ConfigLoadMode::Json(path) => OrchestratorConfig::load_json(path),
    }
}

pub fn save_config_by_mode(
    config: &OrchestratorConfig,
    mode: &ConfigSaveMode,
) -> Result<(), String> {
    match mode {
        ConfigSaveMode::Auto(path) => config.save_auto(path),
        ConfigSaveMode::Ron(path) => config.save_ron(path),
        ConfigSaveMode::Bin(path) => config.save_bin(path),
        ConfigSaveMode::Json(path) => config.save_json(path),
    }
}

pub fn is_binary_config_path(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.trim().to_ascii_lowercase());
    matches!(ext.as_deref(), None | Some("bin"))
}

pub fn validate_orchestrator_config(config: &OrchestratorConfig) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if config.timeout_ms == 0 {
        diagnostics.push(
            "timeout_ms must be > 0 (fix: set --timeout-ms <millis>, e.g. 30000)".to_string(),
        );
    }
    if config.execution_policy.execution_max_iterations == 0 {
        diagnostics.push(
            "execution_max_iterations must be >= 1 (fix: set --execution-max-iterations <count>)"
                .to_string(),
        );
    }
    if config.validation_from_planning_context && config.planning_context_artifact.is_none() {
        diagnostics.push(
            "validation_from_planning_context=true requires planning_context_artifact (fix: set --planning-context-artifact <path>)"
                .to_string(),
        );
    }
    if config.delivery_options.pr_enabled && !config.delivery_options.enabled {
        diagnostics.push(
            "delivery_options.pr_enabled=true requires delivery_options.enabled=true (fix: add --delivery-enabled)"
                .to_string(),
        );
    }
    if config.decision_threshold > 100 {
        diagnostics.push(
            "decision_threshold must be <= 100 (fix: set --decision-threshold 0..100)".to_string(),
        );
    }
    if config.decision_require_contributions && config.decision_contributions.is_empty() {
        diagnostics.push(
            "decision_require_contributions=true requires at least one decision contribution (fix: add --decision-contribution ...)"
                .to_string(),
        );
    }
    diagnostics
}
