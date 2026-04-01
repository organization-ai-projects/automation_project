mod analysis;
mod config;
mod diagnostics;
mod io;
mod linting;
mod neurosymbolic;
mod pipeline;
mod report;

#[cfg(test)]
mod tests;

use diagnostics::AnalyzerError;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: intelligent_analyzer_backend <source_path> [--json-out <path>]");
        std::process::exit(2);
    }

    let source_path = &args[0];
    let json_out = args
        .windows(2)
        .find(|w| w[0] == "--json-out")
        .map(|w| w[1].as_str());

    match run(source_path, json_out) {
        Ok(()) => std::process::exit(0),
        Err(AnalyzerError::InvalidSource(_)) => std::process::exit(3),
        Err(AnalyzerError::PipelineFailure(_)) => std::process::exit(4),
        Err(_) => std::process::exit(5),
    }
}

fn run(source_path: &str, json_out: Option<&str>) -> Result<(), AnalyzerError> {
    let cfg = config::AnalyzerConfig::default();
    let source = io::load_source(source_path)?;
    let result = pipeline::run_pipeline(&cfg, &source)?;
    let output = report::AnalysisReport::from_result(&result);

    match json_out {
        Some(path) => io::write_json_report(&output, path)?,
        None => {
            let json = common_json::to_string_pretty(&output)
                .map_err(|e| AnalyzerError::Io(e.to_string()))?;
            println!("{json}");
        }
    }

    Ok(())
}
