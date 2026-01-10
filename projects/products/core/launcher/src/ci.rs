// projects/products/core/launcher/src/ci.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "core-launcher")]
pub struct Cli {
    /// Path to launcher.toml
    #[arg(long, default_value = "launcher.toml")]
    pub config: String,

    /// Only start these services (comma-separated)
    #[arg(long)]
    pub only: Option<String>,
    /// Skip these services (comma-separated)
    #[arg(long)]
    pub skip: Option<String>,

    /// Do not spawn processes; print what would happen
    #[arg(long)]
    pub dry_run: bool,
    /// Build before launch (overrides config)
    #[arg(long)]
    pub build: bool,

    /// Don't build before launch (overrides config)
    #[arg(long)]
    pub no_build: bool,
}
