mod app;
mod diagnostics;
mod fixtures;
mod public_api;
mod screens;
mod transport;
mod widgets;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    public_api::PublicApi::run(&args);
}
