// projects/products/unstable/hospital_tycoon/backend/src/main.rs
mod config;
mod diagnostics;
mod economy;
mod io;
mod model;
mod patients;
mod protocol;
pub mod public_api;
mod replay;
mod report;
mod reputation;
mod rooms;
mod sim;
mod snapshot;
mod staff;
mod time;
mod triage;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = protocol::server::run(&args);
    if code != 0 {
        std::process::exit(code);
    }
}
