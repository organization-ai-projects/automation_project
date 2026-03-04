mod commands;
mod config;
mod determinism;
mod diagnostics;
mod ecs;
mod events;
mod inspect;
mod io;
mod packs;
mod packs_builtin;
mod protocol;
mod public_api;
mod replay;
mod report;
mod scenario;
mod schedule;
mod snapshot;
mod time;

fn main() {
    tracing_subscriber::fmt().init();
    let args: Vec<String> = std::env::args().collect();
    let exit_code = protocol::server::run(&args);
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
