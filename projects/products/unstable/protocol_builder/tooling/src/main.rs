// projects/products/unstable/protocol_builder/tooling/src/main.rs
mod diagnostics;
mod public_api;
mod validate;

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!(
            "usage: protocol_builder_tooling <validate-emitted|validate-transcript> [options]"
        ));
    }

    match args[1].as_str() {
        "validate-emitted" => {
            let manifest = find_arg(&args, "--manifest")
                .ok_or_else(|| anyhow!("missing --manifest argument"))?;
            public_api::validate_emitted(manifest)?;
        }
        "validate-transcript" => {
            let transcript = find_arg(&args, "--transcript")
                .ok_or_else(|| anyhow!("missing --transcript argument"))?;
            public_api::validate_transcript(transcript)?;
        }
        cmd => {
            return Err(anyhow!("unknown command: {}", cmd));
        }
    }

    Ok(())
}

fn find_arg<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].as_str())
}
