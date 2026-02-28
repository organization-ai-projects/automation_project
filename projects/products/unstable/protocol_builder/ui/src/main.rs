// projects/products/unstable/protocol_builder/ui/src/main.rs
mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    // usage: protocol_builder_ui generate --schema <file> --out <dir>
    if args.len() < 2 || args[1] != "generate" {
        return Err(anyhow!(
            "usage: protocol_builder_ui generate --schema <file> --out <dir>"
        ));
    }

    let schema_path =
        find_arg(&args, "--schema").ok_or_else(|| anyhow!("missing --schema argument"))?;
    let out_dir = find_arg(&args, "--out").ok_or_else(|| anyhow!("missing --out argument"))?;

    // Locate the backend binary alongside this binary
    let self_path = std::env::current_exe()?;
    let backend_binary = self_path
        .parent()
        .map(|p| p.join("protocol_builder_backend"))
        .unwrap_or_else(|| std::path::PathBuf::from("protocol_builder_backend"));

    public_api::run_generate(
        schema_path,
        out_dir,
        backend_binary
            .to_str()
            .unwrap_or("protocol_builder_backend"),
    )?;

    Ok(())
}

fn find_arg<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].as_str())
}
