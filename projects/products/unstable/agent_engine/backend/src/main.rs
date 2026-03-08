// projects/products/unstable/agent_engine/backend/src/main.rs
mod diagnostics;
mod engine;
mod protocol;

use std::{env, process};

use crate::protocol::cli_io;

fn main() {
    if let Err(err) = engine::task_runner::run_cli(env::args().collect()) {
        cli_io::write_error(&err);
        process::exit(1);
    }
}
