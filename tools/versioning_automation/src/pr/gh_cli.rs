pub(crate) fn gh_output_trim(cmd: &str, args: &[&str]) -> Result<String, String> {
    gh_output_with_transform(cmd, args, |stdout| stdout.trim().to_string())
}

pub(crate) fn gh_output_trim_end_newline(cmd: &str, args: &[&str]) -> Result<String, String> {
    gh_output_with_transform(cmd, args, |stdout| {
        stdout.trim_end_matches('\n').to_string()
    })
}

pub(crate) fn gh_status(cmd: &str, args: &[&str]) -> i32 {
    let owned_args = build_args(cmd, args);
    let borrowed_args = owned_args.iter().map(String::as_str).collect::<Vec<&str>>();
    match crate::gh_cli::status(&borrowed_args) {
        Ok(()) => 0,
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
    let owned_args = build_args(cmd, args);
    let borrowed_args = owned_args.iter().map(String::as_str).collect::<Vec<&str>>();
    let stdout = crate::gh_cli::output_preserve(&borrowed_args)?;
    Ok(transform(&stdout))
}

fn build_args(cmd: &str, args: &[&str]) -> Vec<String> {
    let mut out = Vec::with_capacity(args.len() + 1);
    out.push(cmd.to_string());
    out.extend(args.iter().map(|value| (*value).to_string()));
    out
}
