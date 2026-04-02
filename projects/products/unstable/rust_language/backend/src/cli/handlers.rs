//! projects/products/unstable/rust_language/backend/src/cli/handlers.rs
use std::fs;
use std::path;

use crate::ai_assist::CodeImprover;
use crate::ai_assist::TranspileValidator;
use crate::app_error::AppError;
use crate::engine::RhlEngine;
use crate::engine::RonLoader;
use crate::model::ProjectConfig;
use crate::model::SourceFile;

pub(crate) fn handle_compile(config: &ProjectConfig, args: &[String]) -> Result<(), AppError> {
    if args.is_empty() {
        return Err(AppError::Usage("compile requires a .rhl file path"));
    }

    let file_path = &args[0];
    let content = fs::read_to_string(file_path)?;
    let source = SourceFile::new(file_path.clone(), content);

    let eng = RhlEngine::from_config(config.clone());
    eng.compile_source(&source)?;
    Ok(())
}

pub(crate) fn handle_check(args: &[String]) -> Result<(), AppError> {
    if args.is_empty() {
        return Err(AppError::Usage("check requires a .rhl file path"));
    }

    let file_path = &args[0];
    let content = fs::read_to_string(file_path)?;

    let transpiled = RhlEngine::check_source(file_path, &content)?;

    let mut validator = TranspileValidator::new()?;
    validator.validate_transpilation(&content, "")?;

    Ok(())
}

pub(crate) fn handle_init(config: &ProjectConfig, args: &[String]) -> Result<(), AppError> {
    if args.is_empty() {
        return Err(AppError::Usage("init requires a project name"));
    }

    let name = &args[0];
    let config_path = format!("{name}.ron");
    RonLoader::save_config(path::Path::new(&config_path), config)?;
    Ok(())
}

pub(crate) fn improve_code(args: &[String]) -> Result<String, AppError> {
    if args.is_empty() {
        return Err(AppError::Usage("improve_code requires code as input"));
    }

    let mut improver = CodeImprover::new()?;

    let code = &args[0];
    let lang = find_flag_value(args, "--lang").unwrap_or_else(|| "rhl".to_string());

    let result = match lang.as_str() {
        "rhl" => improver.improve_rhl_code(code)?,
        "rust" => improver.optimize_transpiled_rust(code)?,
        _ => return Err(AppError::Usage("Unsupported language specified")),
    };

    Ok(result)
}

pub(crate) fn handle_save_config(args: &[String]) -> Result<(), AppError> {
    if args.len() < 2 {
        return Err(AppError::Usage(
            "save_config requires a config path and a save path",
        ));
    }

    let config_path = path::Path::new(&args[0]);
    let save_path = path::Path::new(&args[1]);

    let engine = RhlEngine::from_ron(config_path)?;
    engine.save_config(save_path)?;

    Ok(())
}

pub(crate) fn handle_save_binary(config: &ProjectConfig, args: &[String]) -> Result<(), AppError> {
    if args.len() < 2 {
        return Err(AppError::Usage(
            "save_binary requires a source file path and an output path",
        ));
    }

    let source_path = &args[0];
    let output_path = path::Path::new(&args[1]);

    let content = fs::read_to_string(source_path)?;

    let source = SourceFile::new(source_path.clone(), content);
    let engine = RhlEngine::from_config(config.clone());

    engine.save_binary(&source, output_path)?;

    Ok(())
}

pub(crate) fn handle_parse_to_ast(
    config: &ProjectConfig,
    args: &[String],
) -> Result<String, AppError> {
    if args.is_empty() {
        return Err(AppError::Usage(
            "parse_to_ast requires source code as input",
        ));
    }

    let source_code = &args[0];
    let engine = RhlEngine::from_config(config.clone());

    let ast = engine.parse_to_ast(source_code)?;

    common_json::to_json_string_pretty(&ast)
        .map_err(|e| AppError::Internal(format!("Failed to serialize AST: {}", e)))
}

fn find_flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).cloned())
}
