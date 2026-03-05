mod config;
mod diagnostics;
mod events;
mod hauling;
mod io;
mod jobs;
mod map;
mod model;
mod moods;
mod needs;
mod protocol;
mod public_api;
mod replay;
mod report;
mod rng;
mod scenarios;
mod sim_engine;
mod snapshot;
mod time;

use std::{env, process};

use crate::protocol::server;

fn main() {
    process::exit(server::run(env::args().collect()));
}
