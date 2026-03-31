mod antivirus;
mod app;
mod diagnostics;
mod firewall;
mod io;
mod moe_protect;
mod protocol;
mod symbolic_engine;

#[cfg(test)]
mod tests;

fn main() {
    if let Err(e) = app::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
