mod app;
mod diagnostics;
mod public_api;
mod screens;
mod transport;
mod widgets;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exit_code = match public_api::run_cli(&args) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error}");
            2
        }
    };
    std::process::exit(exit_code);
}
