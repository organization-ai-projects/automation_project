mod ai_assist;
mod compiler;
mod diagnostics;
mod engine;
mod model;

#[cfg(test)]
mod tests;

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("");

    let result = match command {
        "compile" => handle_compile(&args[2..]),
        "check" => handle_check(&args[2..]),
        "init" => handle_init(&args[2..]),
        _ => {
            eprintln!("Usage: rust_language_backend <compile|check|init> [OPTIONS]");
            eprintln!("  compile <file.rhl>              Transpile .rhl to Rust");
            eprintln!("  compile <file.rhl> --binary <out>  Compile to binary");
            eprintln!("  check <file.rhl>                Check for errors with AI");
            eprintln!("  init <name>                     Create new project config");
            process::exit(2);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn handle_compile(args: &[String]) -> Result<(), diagnostics::error::Error> {
    if args.is_empty() {
        return Err(diagnostics::error::Error::InvalidCli(
            "compile requires a .rhl file path".into(),
        ));
    }

    let file_path = &args[0];
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| diagnostics::error::Error::Io(e.to_string()))?;
    let source = model::source_file::SourceFile::new(file_path.clone(), content);

    let config = model::project_config::ProjectConfig::new(
        "inline".into(),
        "0.1.0".into(),
        file_path.clone(),
    );
    let eng = engine::rhl_engine::RhlEngine::from_config(config);

    let binary_out = find_flag_value(args, "--binary");

    if let Some(out_path) = binary_out {
        eng.save_binary(&source, std::path::Path::new(&out_path))?;
        eprintln!("Binary written to {out_path}");
    } else {
        let rust_code = eng.compile_source(&source)?;
        println!("{rust_code}");
    }
    Ok(())
}

fn handle_check(args: &[String]) -> Result<(), diagnostics::error::Error> {
    if args.is_empty() {
        return Err(diagnostics::error::Error::InvalidCli(
            "check requires a .rhl file path".into(),
        ));
    }

    let file_path = &args[0];
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| diagnostics::error::Error::Io(e.to_string()))?;
    let source = model::source_file::SourceFile::new(file_path.clone(), content.clone());

    let config = model::project_config::ProjectConfig::new(
        "inline".into(),
        "0.1.0".into(),
        file_path.clone(),
    );
    let eng = engine::rhl_engine::RhlEngine::from_config(config);

    match eng.compile_source(&source) {
        Ok(rust_code) => {
            println!("Compilation successful. Transpiled Rust:\n{rust_code}");
            let mut validator =
                ai_assist::transpile_validator::TranspileValidator::new()?;
            let analysis = validator.validate_transpilation(&content, &rust_code)?;
            println!("\nAI Validation:\n{analysis}");
        }
        Err(e) => {
            eprintln!("Compilation failed: {e}");
            let mut analyzer = ai_assist::error_analyzer::ErrorAnalyzer::new()?;
            let analysis = analyzer.analyze_compilation_error(&content, &e.to_string())?;
            println!("\nAI Error Analysis:\n{analysis}");
            let fix = analyzer.suggest_fix(&content, &e.to_string())?;
            println!("\nSuggested Fix:\n{fix}");
        }
    }
    Ok(())
}

fn handle_init(args: &[String]) -> Result<(), diagnostics::error::Error> {
    if args.is_empty() {
        return Err(diagnostics::error::Error::InvalidCli(
            "init requires a project name".into(),
        ));
    }

    let name = &args[0];
    let config = model::project_config::ProjectConfig::new(
        name.clone(),
        "0.1.0".into(),
        "main.rhl".into(),
    );
    let config_path = format!("{name}.ron");
    engine::ron_loader::RonLoader::save_config(std::path::Path::new(&config_path), &config)?;
    eprintln!("Project config written to {config_path}");
    Ok(())
}

fn find_flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}
