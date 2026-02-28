use std::path::PathBuf;

/// Parsed CLI arguments for the `workflow_offline` binary.
pub struct CliArgs {
    /// Path to the workflow TOML file.
    pub workflow_file: PathBuf,
    /// Seed for deterministic ordering.
    pub seed: u64,
    /// Validate and print execution order without running commands.
    pub dry_run: bool,
    /// Print `RunReport` as JSON to stdout on success.
    pub json: bool,
}

/// Parses `std::env::args()`.
///
/// Returns `Ok(CliArgs)` on success or `Err(message)` on invalid usage.
/// Exit code 2 is used by the caller for parse errors.
pub fn parse_args() -> Result<CliArgs, String> {
    let args: Vec<String> = std::env::args().collect();
    parse_args_from(&args)
}

pub fn parse_args_from(args: &[String]) -> Result<CliArgs, String> {
    // Expected: workflow_offline run <workflow_file> [--seed <u64>] [--dry-run] [--json]
    if args.len() < 2 {
        return Err(usage());
    }
    if args[1] != "run" {
        return Err(format!("unknown command `{}`; {}", args[1], usage()));
    }
    if args.len() < 3 {
        return Err(format!("missing <workflow_file>; {}", usage()));
    }

    let workflow_file = PathBuf::from(&args[2]);
    let mut seed: u64 = 0;
    let mut dry_run = false;
    let mut json = false;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                i += 1;
                let val = args
                    .get(i)
                    .ok_or_else(|| "--seed requires a value".to_string())?;
                seed = val
                    .parse::<u64>()
                    .map_err(|_| format!("--seed value `{val}` is not a valid u64"))?;
            }
            "--dry-run" => dry_run = true,
            "--json" => json = true,
            other => return Err(format!("unknown flag `{other}`; {}", usage())),
        }
        i += 1;
    }

    Ok(CliArgs {
        workflow_file,
        seed,
        dry_run,
        json,
    })
}

fn usage() -> String {
    "Usage: workflow_offline run <workflow_file> [--seed <u64>] [--dry-run] [--json]".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn minimal_run_command() {
        let a = parse_args_from(&args(&["wf", "run", "my.toml"])).unwrap();
        assert_eq!(a.workflow_file, PathBuf::from("my.toml"));
        assert_eq!(a.seed, 0);
        assert!(!a.dry_run);
        assert!(!a.json);
    }

    #[test]
    fn all_flags() {
        let a = parse_args_from(&args(&[
            "wf", "run", "my.toml", "--seed", "42", "--dry-run", "--json",
        ]))
        .unwrap();
        assert_eq!(a.seed, 42);
        assert!(a.dry_run);
        assert!(a.json);
    }

    #[test]
    fn unknown_command_returns_error() {
        assert!(parse_args_from(&args(&["wf", "help"])).is_err());
    }

    #[test]
    fn missing_workflow_file_returns_error() {
        assert!(parse_args_from(&args(&["wf", "run"])).is_err());
    }

    #[test]
    fn invalid_seed_returns_error() {
        assert!(parse_args_from(&args(&["wf", "run", "f.toml", "--seed", "abc"])).is_err());
    }

    #[test]
    fn missing_seed_value_returns_error() {
        assert!(parse_args_from(&args(&["wf", "run", "f.toml", "--seed"])).is_err());
    }
}
