mod ability;
mod ai;
mod config;
mod controller;
mod diagnostics;
mod grid;
mod io;
mod protocol;
mod replay;
mod report;
mod rng;
mod scenario;
mod snapshot;
mod turn;
mod unit;

#[cfg(test)]
mod tests;

use std::{env, process};
use crate::protocol::server;

fn main() {
    process::exit(server::run(env::args().collect()));
}
