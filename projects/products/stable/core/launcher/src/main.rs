// projects/products/core/launcher/src/main.rs
mod build;
mod cargo_commands;
mod child_handle;
mod ci;
mod config;
mod defaults;
mod entry;
mod launcher;
mod logging;
mod normalizer;
mod parsing;
mod process;
mod restart_policy;
mod service;
mod shutdown;
mod supervisor;
mod workspace;

fn main() -> anyhow::Result<()> {
    entry::main()
}
