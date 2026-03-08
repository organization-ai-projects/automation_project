mod app;
mod diagnostics;
mod screens;
mod transport;
mod widgets;

use app::controller::{resolve_backend_binary_path, run_flow};
use diagnostics::error::UiError;

fn main() -> Result<(), UiError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        return Err(UiError::MissingArgument(
            "usage: schema_world_ui <schema.json> <record.json>",
        ));
    }

    let backend = resolve_backend_binary_path()?;
    run_flow(&args[1], &args[2], &backend)
}
