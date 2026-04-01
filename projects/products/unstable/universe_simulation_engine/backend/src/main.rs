mod config;
mod controller;
mod cosmology;
mod diagnostics;
mod io;
mod math;
mod particles;
mod physics;
mod protocol;
mod report;
mod rng;
mod sim;
mod snapshot;
mod spatial;
mod structures;
mod time;

use std::{env, process};

use crate::protocol::server;

fn main() {
    process::exit(server::run(env::args().collect()));
}

#[cfg(test)]
mod tests;
