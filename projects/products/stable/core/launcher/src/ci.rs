// projects/products/stable/core/launcher/src/ci.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "core-launcher")]
pub(crate) struct Cli {
    /// Path to launcher.toml
    #[arg(long, default_value = "launcher.toml")]
    pub(crate) config: String,

    /// Only start these services (comma-separated)
    #[arg(long)]
    pub(crate) only: Option<String>,
    /// Skip these services (comma-separated)
    #[arg(long)]
    pub(crate) skip: Option<String>,

    /// Do not spawn processes; print what would happen
    #[arg(long)]
    pub(crate) dry_run: bool,
    /// Build before launch (overrides config)
    #[arg(long)]
    pub(crate) build: bool,

    /// Don't build before launch (overrides config)
    #[arg(long)]
    pub(crate) no_build: bool,
}
