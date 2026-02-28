mod diagnostics;
mod flow;
mod model;
mod persistence;
mod replay;
mod sim;

pub mod public_api;

#[cfg(test)]
mod tests;

fn main() {
    println!("factory_sim: deterministic tick-based factory simulator");
}
