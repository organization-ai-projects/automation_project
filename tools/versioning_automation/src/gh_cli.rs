//! tools/versioning_automation/src/gh_cli.rs
use std::process::Command;

pub(crate) fn command(args: &[&str]) -> Command {
    let mut command = Command::new("gh");
    command.args(args);
    command
}

pub(crate) fn output_trim(args: &[&str]) -> Result<String, String> {
    output_with_transform(args, |stdout| stdout.trim().to_string())
}

pub(crate) fn output_preserve(args: &[&str]) -> Result<String, String> {
    output_with_transform(args, |stdout| stdout.to_string())
}

pub(crate) fn output_trim_cmd(cmd: &str, args: &[&str]) -> Result<String, String> {
    let owned_args = build_args(cmd, args);
    let borrowed_args = owned_args.iter().map(String::as_str).collect::<Vec<&str>>();
    output_trim(&borrowed_args)
}

pub(crate) fn output_preserve_cmd(cmd: &str, args: &[&str]) -> Result<String, String> {
    let owned_args = build_args(cmd, args);
    let borrowed_args = owned_args.iter().map(String::as_str).collect::<Vec<&str>>();
    output_preserve(&borrowed_args)
}

pub(crate) fn output_trim_end_newline_cmd(cmd: &str, args: &[&str]) -> Result<String, String> {
    let stdout = output_preserve_cmd(cmd, args)?;
    Ok(stdout.trim_end_matches('\n').to_string())
}

pub(crate) fn status(args: &[&str]) -> Result<(), String> {
    let status = command(args).status().map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "gh {} failed with exit {:?}",
            args.join(" "),
            status.code()
        ))
    }
}

pub(crate) fn status_cmd(cmd: &str, args: &[&str]) -> Result<(), String> {
    let owned_args = build_args(cmd, args);
    let borrowed_args = owned_args.iter().map(String::as_str).collect::<Vec<&str>>();
    status(&borrowed_args)
}

pub(crate) fn status_owned(args: &[String]) -> Result<(), String> {
    let borrowed_args = args.iter().map(String::as_str).collect::<Vec<&str>>();
    status(&borrowed_args)
}

fn output_with_transform(args: &[&str], transform: fn(&str) -> String) -> Result<String, String> {
    let output = command(args).output().map_err(|err| err.to_string())?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(transform(&stdout))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn build_args(cmd: &str, args: &[&str]) -> Vec<String> {
    let mut out = Vec::with_capacity(args.len() + 1);
    out.push(cmd.to_string());
    out.extend(args.iter().map(|value| (*value).to_string()));
    out
}
