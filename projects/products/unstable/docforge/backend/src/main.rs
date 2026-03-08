mod app;
mod diagnostics;
mod edit;
mod layout;
mod model;
mod persistence;
mod protocol;
mod render;
mod replay;

fn main() {
    if app::run().is_err() {
        std::process::exit(1);
    }
}
