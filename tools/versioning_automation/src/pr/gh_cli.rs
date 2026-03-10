use std::process::Command;

pub(crate) fn gh_output_trim(cmd: &str, args: &[&str]) -> Result<String, String> {
    gh_output_with_transform(cmd, args, |stdout| stdout.trim().to_string())
}

pub(crate) fn gh_output_trim_end_newline(cmd: &str, args: &[&str]) -> Result<String, String> {
    gh_output_with_transform(cmd, args, |stdout| {
        stdout.trim_end_matches('\n').to_string()
    })
}

pub(crate) fn gh_status(cmd: &str, args: &[&str]) -> i32 {
    let mut command = Command::new("gh");
    command.arg(cmd);
    for arg in args {
        command.arg(arg);
    }
    match command.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(err) => {
            eprintln!("Failed to execute gh {}: {err}", cmd);
            1
        }
    }
}

fn gh_output_with_transform(
    cmd: &str,
    args: &[&str],
    transform: fn(&str) -> String,
) -> Result<String, String> {
    let mut command = Command::new("gh");
    command.arg(cmd);
    for arg in args {
        command.arg(arg);
    }
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                Ok(transform(&text))
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
