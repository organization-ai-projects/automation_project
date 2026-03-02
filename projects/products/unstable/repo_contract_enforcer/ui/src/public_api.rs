// projects/products/unstable/repo_contract_enforcer/ui/src/public_api.rs
use anyhow::Result;

use crate::cli::args::Args;
use crate::cli::command::{Command, Mode};
use crate::render::human_printer::HumanPrinter;
use crate::render::json_printer::JsonPrinter;
use crate::render::vscode_printer::VscodePrinter;
use crate::transport::ipc_client::{
    IpcClient, Report, ReportMode, RequestPayload, ResponsePayload,
};

pub fn run_cli(args: &[String]) -> Result<i32> {
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
            if parsed.vscode {
                VscodePrinter::print_report(&report_json);
            } else if parsed.json {
                JsonPrinter::print_report(&report_json)?;
            } else {
                HumanPrinter::print_report(&report_json);
            }
            exit_code = strict_blocking_exit_code(&report_json);
        }
        ResponsePayload::Error { code, message, .. } => {
            eprintln!("backend error [{code}]: {message}");
            exit_code = 5;
        }
        ResponsePayload::Ok => {
            if !parsed.json && !parsed.vscode {
                println!("ok");
            }
        }
    }

    client.close();
    Ok(exit_code)
}

fn strict_blocking_exit_code(report: &Report) -> i32 {
    if report.mode == ReportMode::Strict && report.summary.stable_error_count > 0 {
        3
    } else {
        0
    }
}

pub(crate) fn parse_args(args: &[String]) -> Result<Args> {
    if args.len() < 2 {
        anyhow::bail!(
            "usage: repo_contract_enforcer_ui check --root <path> [--mode auto|strict|relaxed] [--json|--vscode]"
        )
    }

    let cmd = args[1].as_str();
    let mut json = false;
    let mut vscode = false;
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
            "--vscode" => {
                vscode = true;
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

    if json && vscode {
        anyhow::bail!("--json and --vscode are mutually exclusive");
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

    Ok(Args {
        json,
        vscode,
        command,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_args(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parse_check_defaults_root_and_mode_auto() {
        let args = vec_args(&["repo_contract_enforcer_ui", "check"]);
        let parsed = parse_args(&args).expect("parse check");
        assert!(!parsed.json);
        assert!(!parsed.vscode);
        match parsed.command {
            Command::Check { root, mode } => {
                assert_eq!(root, ".");
                assert_eq!(mode, Mode::Auto);
            }
            _ => panic!("expected check command"),
        }
    }

    #[test]
    fn parse_check_product_requires_path() {
        let args = vec_args(&["repo_contract_enforcer_ui", "check-product"]);
        let err = parse_args(&args).expect_err("missing path should fail");
        assert!(err.to_string().contains("--path"));
    }

    #[test]
    fn parse_check_product_with_mode_and_json() {
        let args = vec_args(&[
            "repo_contract_enforcer_ui",
            "check-product",
            "--path",
            "projects/products/unstable/repo_contract_enforcer",
            "--mode",
            "strict",
            "--json",
        ]);
        let parsed = parse_args(&args).expect("parse check-product");
        assert!(parsed.json);
        assert!(!parsed.vscode);
        match parsed.command {
            Command::CheckProduct { path, mode } => {
                assert_eq!(path, "projects/products/unstable/repo_contract_enforcer");
                assert_eq!(mode, Mode::Strict);
            }
            _ => panic!("expected check-product command"),
        }
    }

    #[test]
    fn parse_check_with_vscode_flag() {
        let args = vec_args(&[
            "repo_contract_enforcer_ui",
            "check",
            "--root",
            ".",
            "--vscode",
        ]);
        let parsed = parse_args(&args).expect("parse check --vscode");
        assert!(!parsed.json);
        assert!(parsed.vscode);
    }

    #[test]
    fn parse_rejects_json_and_vscode_together() {
        let args = vec_args(&["repo_contract_enforcer_ui", "check", "--json", "--vscode"]);
        let err = parse_args(&args).expect_err("both flags must fail");
        assert!(err.to_string().contains("mutually exclusive"));
    }

    #[test]
    fn strict_mode_with_stable_errors_returns_3() {
        let report = Report {
            repository_root: ".".to_string(),
            mode: ReportMode::Strict,
            violations: Vec::new(),
            summary: crate::transport::ipc_client::ReportSummary {
                stable_error_count: 1,
                stable_warning_count: 0,
                unstable_error_count: 0,
                unstable_warning_count: 0,
            },
            report_hash: "h".to_string(),
        };
        assert_eq!(strict_blocking_exit_code(&report), 3);
    }

    #[test]
    fn relaxed_mode_never_blocks_even_with_errors() {
        let report = Report {
            repository_root: ".".to_string(),
            mode: ReportMode::Relaxed,
            violations: Vec::new(),
            summary: crate::transport::ipc_client::ReportSummary {
                stable_error_count: 2,
                stable_warning_count: 0,
                unstable_error_count: 1,
                unstable_warning_count: 0,
            },
            report_hash: "h".to_string(),
        };
        assert_eq!(strict_blocking_exit_code(&report), 0);
    }
}
