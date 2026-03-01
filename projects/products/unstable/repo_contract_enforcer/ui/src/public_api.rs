use anyhow::Result;

use crate::cli::args::Args;
use crate::cli::command::{Command, Mode};
use crate::render::human_printer::HumanPrinter;
use crate::render::json_printer::JsonPrinter;
use crate::transport::ipc_client::{IpcClient, RequestPayload, ResponsePayload};

pub fn run_cli(args: &[String]) -> Result<()> {
    let parsed = parse_args(args)?;

    let mut client = IpcClient::connect()?;
    let response = match parsed.command.clone() {
        Command::Check { root, mode } => client.request_report(RequestPayload::CheckRepo {
            root_path: root,
            mode,
        })?,
        Command::CheckProduct { path, mode } => {
            client.request_report(RequestPayload::CheckProduct {
                product_path: path,
                mode,
            })?
        }
    };

    let mut exit_code = 0;

    match response.payload {
        ResponsePayload::Report { report_json, .. } => {
            if parsed.json {
                JsonPrinter::print_report(&report_json)?;
            } else {
                HumanPrinter::print_report(&report_json);
            }

            if let Some(mode) = report_json.get("mode").and_then(|m| m.as_str())
                && mode == "strict"
            {
                let stable_errors = report_json
                    .get("summary")
                    .and_then(|s| s.get("stable_error_count"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                if stable_errors > 0 {
                    exit_code = 3;
                }
            }
        }
        ResponsePayload::Error { code, message, .. } => {
            eprintln!("backend error [{code}]: {message}");
            exit_code = 5;
        }
        ResponsePayload::Ok => {
            if !parsed.json {
                println!("ok");
            }
        }
    }

    client.close();
    std::process::exit(exit_code);
}

fn parse_args(args: &[String]) -> Result<Args> {
    if args.len() < 2 {
        anyhow::bail!(
            "usage: repo_contract_enforcer_ui check --root <path> [--mode auto|strict|relaxed] [--json]"
        )
    }

    let cmd = args[1].as_str();
    let mut json = false;
    let mut mode = Mode::Auto;
    let mut root: Option<String> = None;
    let mut product_path: Option<String> = None;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                json = true;
                i += 1;
            }
            "--mode" => {
                let v = args
                    .get(i + 1)
                    .ok_or_else(|| anyhow::anyhow!("--mode requires a value"))?;
                mode = match v.as_str() {
                    "auto" => Mode::Auto,
                    "strict" => Mode::Strict,
                    "relaxed" => Mode::Relaxed,
                    _ => anyhow::bail!("invalid mode: {v}"),
                };
                i += 2;
            }
            "--root" => {
                root = Some(
                    args.get(i + 1)
                        .ok_or_else(|| anyhow::anyhow!("--root requires a value"))?
                        .clone(),
                );
                i += 2;
            }
            "--path" => {
                product_path = Some(
                    args.get(i + 1)
                        .ok_or_else(|| anyhow::anyhow!("--path requires a value"))?
                        .clone(),
                );
                i += 2;
            }
            other => anyhow::bail!("unknown argument: {other}"),
        }
    }

    let command = match cmd {
        "check" => Command::Check {
            root: root.unwrap_or_else(|| ".".to_string()),
            mode,
        },
        "check-product" => Command::CheckProduct {
            path: product_path
                .ok_or_else(|| anyhow::anyhow!("check-product requires --path <product_path>"))?,
            mode,
        },
        _ => anyhow::bail!("unknown command: {cmd}"),
    };

    Ok(Args { json, command })
}
