// projects/products/unstable/autonomy_orchestrator_ai/src/fixture.rs
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

pub fn run(args: &[String]) -> ! {
    let Some(mode) = args.first().map(String::as_str) else {
        usage_and_exit();
    };
    match mode {
        "success" => process::exit(0),
        "fail" => process::exit(1),
        "sleep-ms" => {
            if args.len() != 2 {
                usage_and_exit();
            }
            let ms = args[1].parse::<u64>().unwrap_or_else(|_| {
                eprintln!("Invalid sleep-ms value: {}", args[1]);
                process::exit(2);
            });
            thread::sleep(Duration::from_millis(ms));
            process::exit(0);
        }
        "fail-once" => {
            if args.len() != 2 {
                usage_and_exit();
            }
            let state = PathBuf::from(args[1].clone());
            if state.exists() {
                process::exit(0);
            }
            if let Some(parent) = state.parent()
                && !parent.as_os_str().is_empty()
            {
                fs::create_dir_all(parent).unwrap_or_else(|err| {
                    eprintln!("Failed to create state parent directory: {err}");
                    process::exit(1);
                });
            }
            fs::write(&state, b"first-failure").unwrap_or_else(|err| {
                eprintln!("Failed to write state file '{}': {err}", state.display());
                process::exit(1);
            });
            process::exit(1);
        }
        "review-remediation" => {
            if args.len() != 3 {
                usage_and_exit();
            }
            let state = PathBuf::from(args[1].clone());
            let report = PathBuf::from(args[2].clone());
            if let Some(parent) = report.parent()
                && !parent.as_os_str().is_empty()
            {
                fs::create_dir_all(parent).unwrap_or_else(|err| {
                    eprintln!("Failed to create report parent directory: {err}");
                    process::exit(1);
                });
            }

            if state.exists() {
                fs::write(
                    &report,
                    r#"{"next_step_plan":[{"priority":1,"code":"DONE","action":"No action"}]}"#,
                )
                .unwrap_or_else(|err| {
                    eprintln!(
                        "Failed to write review report '{}': {err}",
                        report.display()
                    );
                    process::exit(1);
                });
                process::exit(0);
            }

            if let Some(parent) = state.parent()
                && !parent.as_os_str().is_empty()
            {
                fs::create_dir_all(parent).unwrap_or_else(|err| {
                    eprintln!("Failed to create state parent directory: {err}");
                    process::exit(1);
                });
            }
            fs::write(&state, b"remediation-required").unwrap_or_else(|err| {
                eprintln!("Failed to write state file '{}': {err}", state.display());
                process::exit(1);
            });
            fs::write(
                &report,
                r#"{"next_step_plan":[{"priority":1,"code":"FIX_VALIDATION","action":"Rerun execution with reviewer feedback"}]}"#,
            )
            .unwrap_or_else(|err| {
                eprintln!("Failed to write review report '{}': {err}", report.display());
                process::exit(1);
            });
            process::exit(1);
        }
        "write-file" => {
            if args.len() != 3 {
                usage_and_exit();
            }
            let path = PathBuf::from(args[1].clone());
            let content = args[2].clone();
            if let Some(parent) = path.parent()
                && !parent.as_os_str().is_empty()
            {
                fs::create_dir_all(parent).unwrap_or_else(|err| {
                    eprintln!("Failed to create file parent directory: {err}");
                    process::exit(1);
                });
            }
            fs::write(&path, content).unwrap_or_else(|err| {
                eprintln!("Failed to write file '{}': {err}", path.display());
                process::exit(1);
            });
            process::exit(0);
        }
        "assert-file-contains" => {
            if args.len() != 3 {
                usage_and_exit();
            }
            let path = PathBuf::from(args[1].clone());
            let needle = args[2].clone();
            let content = fs::read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Failed to read file '{}': {err}", path.display());
                process::exit(1);
            });
            if content.contains(&needle) {
                process::exit(0);
            }
            eprintln!(
                "Expected file '{}' to contain '{}', but it did not",
                path.display(),
                needle
            );
            process::exit(1);
        }
        _ => usage_and_exit(),
    }
}

fn usage_and_exit() -> ! {
    eprintln!("Usage:");
    eprintln!("  autonomy_orchestrator_ai fixture success");
    eprintln!("  autonomy_orchestrator_ai fixture fail");
    eprintln!("  autonomy_orchestrator_ai fixture sleep-ms <milliseconds>");
    eprintln!("  autonomy_orchestrator_ai fixture fail-once <state_file>");
    eprintln!(
        "  autonomy_orchestrator_ai fixture review-remediation <state_file> <review_report_path>"
    );
    eprintln!("  autonomy_orchestrator_ai fixture write-file <path> <content>");
    eprintln!("  autonomy_orchestrator_ai fixture assert-file-contains <path> <needle>");
    process::exit(2);
}
