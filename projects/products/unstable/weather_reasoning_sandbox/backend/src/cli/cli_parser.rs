use crate::cli::cli_error::CliError;
use crate::cli::output_mode::OutputMode;
use crate::cli::replay_args::ReplayArgs;
use crate::cli::run_args::RunArgs;

pub enum CliCommand {
    Run(RunArgs),
    Replay(ReplayArgs),
}

pub struct CliParser;

impl CliParser {
    pub fn parse(args: &[String]) -> Result<CliCommand, CliError> {
        let command = args
            .get(1)
            .map(String::as_str)
            .ok_or_else(|| CliError::MissingArgument("command (run|replay)".to_string()))?;

        match command {
            "run" => Self::parse_run(&args[2..]),
            "replay" => Self::parse_replay(&args[2..]),
            other => Err(CliError::InvalidArgument(format!(
                "Unknown command: {other}. Expected 'run' or 'replay'"
            ))),
        }
    }

    fn parse_run(args: &[String]) -> Result<CliCommand, CliError> {
        let mut ticks: u64 = 10;
        let mut seed: u64 = 42;
        let mut dataset: Option<String> = None;
        let mut output_mode = OutputMode::Text;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--ticks" => {
                    i += 1;
                    ticks = Self::parse_u64(args.get(i), "--ticks")?;
                }
                "--seed" => {
                    i += 1;
                    seed = Self::parse_u64(args.get(i), "--seed")?;
                }
                "--dataset" => {
                    i += 1;
                    dataset = Some(
                        args.get(i)
                            .ok_or_else(|| {
                                CliError::MissingArgument("--dataset value".to_string())
                            })?
                            .clone(),
                    );
                }
                "--json" => {
                    output_mode = OutputMode::Json;
                }
                _ => {}
            }
            i += 1;
        }

        Ok(CliCommand::Run(RunArgs {
            ticks,
            seed,
            dataset,
            output_mode,
        }))
    }

    fn parse_replay(args: &[String]) -> Result<CliCommand, CliError> {
        let replay_file = args
            .first()
            .ok_or_else(|| CliError::MissingArgument("replay file path".to_string()))?
            .clone();

        let output_mode = if args.iter().any(|a| a == "--json") {
            OutputMode::Json
        } else {
            OutputMode::Text
        };

        Ok(CliCommand::Replay(ReplayArgs {
            replay_file,
            output_mode,
        }))
    }

    fn parse_u64(val: Option<&String>, flag: &str) -> Result<u64, CliError> {
        val.ok_or_else(|| CliError::MissingArgument(format!("{flag} requires a value")))?
            .parse::<u64>()
            .map_err(|_| CliError::InvalidArgument(format!("{flag} must be a positive integer")))
    }
}
