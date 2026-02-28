mod diagnostics;
mod engine;
mod explain;
mod formula;
mod model;
mod persistence;
pub mod public_api;
#[cfg(test)]
mod tests;

pub use public_api::*;

fn main() {
    println!("living_spreadsheet: graph-based deterministic spreadsheet engine");
}
