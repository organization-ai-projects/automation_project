mod actions;
mod app;
mod debate;
mod diagnostics;
mod events;
mod model;
mod poll;
mod protocol;
mod replay;
mod report;
mod sim;

#[cfg(test)]
mod tests;

fn main() {
    tracing_subscriber::fmt::init();
    if let Err(e) = app::run(std::env::args().collect()) {
        protocol::console::print_error_line(&format!("Error: {e}"));
        std::process::exit(1);
    }
}
