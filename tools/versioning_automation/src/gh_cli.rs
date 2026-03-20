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

fn output_with_transform(args: &[&str], transform: fn(&str) -> String) -> Result<String, String> {
    let output = command(args).output().map_err(|err| err.to_string())?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(transform(&stdout))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}
