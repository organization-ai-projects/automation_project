//! tools/versioning_automation/src/git_cli.rs
use std::process::Command;

pub(crate) fn output_trim(args: &[&str]) -> Result<String, String> {
    let output = command(args).output().map_err(|err| err.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

pub(crate) fn output_preserve(args: &[&str]) -> Result<String, String> {
    let output = command(args).output().map_err(|err| err.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

pub(crate) fn status(args: &[&str]) -> Result<(), String> {
    let status = command(args).status().map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "git {} failed with exit {:?}",
            args.join(" "),
            status.code()
        ))
    }
}

pub(crate) fn status_success(args: &[&str]) -> bool {
    command(args)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub(crate) fn command(args: &[&str]) -> Command {
    let mut command = Command::new("git");
    command.args(args);
    command
}
