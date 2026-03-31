mod compile;
mod diagnostics;
mod dsl;
mod io;
mod output;
mod protocol;
mod spec;
mod validate;

use compile::project_emitter::ProjectEmitter;
use compile::runtime_emitter::RuntimeEmitter;
use compile::test_emitter::TestEmitter;
use diagnostics::error::CompilerError;
use dsl::parser::Parser;
use io::fs_writer::FsWriter;
use output::artifact_manifest::{ArtifactManifest, ManifestEntry};
use output::compile_report::CompileReport;
use protocol::message::{IpcMessage, IpcPayload};
use protocol::request::CompilerRequest;
use protocol::response::CompilerResponse;
use sha2::{Digest, Sha256};
use spec::spec::RuntimeSpec;
use std::io::{BufRead, Write};
use validate::determinism_checker::DeterminismChecker;
use validate::invariant_checker::InvariantChecker;

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(0);
    }

    match args[1].as_str() {
        "compile" => run_compile(&args),
        "validate" => run_validate(&args),
        "dry-run" => run_dry_run(&args),
        "serve" => run_serve(),
        _ => {
            eprintln!("error: unknown command '{}'", args[1]);
            print_usage();
            std::process::exit(2);
        }
    }
}

fn run_compile(args: &[String]) {
    let dsl_src = args.get(2).cloned().unwrap_or_default();
    let out_dir = args.get(3).cloned().unwrap_or_else(|| "out".to_string());

    tracing::info!(dsl = %dsl_src, out = %out_dir, "compile started");

    match compile_pipeline(&dsl_src, &out_dir, false) {
        Ok(report) => {
            let json = common_json::to_string(&report).unwrap_or_default();
            println!("{json}");
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn run_validate(args: &[String]) {
    let dsl_src = args.get(2).cloned().unwrap_or_default();
    tracing::info!(dsl = %dsl_src, "validate started");

    match compile_pipeline(&dsl_src, "", true) {
        Ok(report) => {
            let json = common_json::to_string(&report).unwrap_or_default();
            println!("{json}");
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn run_dry_run(args: &[String]) {
    let dsl_src = args.get(2).cloned().unwrap_or_default();
    tracing::info!(dsl = %dsl_src, "dry-run started");

    match compile_pipeline(&dsl_src, "", true) {
        Ok(report) => {
            tracing::info!(
                artifacts = report.artifact_count,
                manifest_hash = %report.manifest_hash,
                "dry-run complete"
            );
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn run_serve() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut session = SessionState::default();

    for line_result in stdin.lock().lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("error: stdin read failed: {e}");
                break;
            }
        };
        if line.trim().is_empty() {
            continue;
        }

        let request_message: Result<IpcMessage, _> = io::json_codec::decode(&line);
        let (id, response) = match request_message {
            Ok(msg) => {
                let response = match msg.payload {
                    IpcPayload::Request(req) => handle_request(req, &mut session),
                    IpcPayload::Response(_) => CompilerResponse::Error {
                        message: "unexpected response payload from client".to_string(),
                    },
                };
                (msg.id, response)
            }
            Err(e) => (
                0,
                CompilerResponse::Error {
                    message: format!("invalid IPC message: {e}"),
                },
            ),
        };

        let response_message = IpcMessage {
            id,
            payload: IpcPayload::Response(response),
        };
        match io::json_codec::encode(&response_message) {
            Ok(json) => {
                if writeln!(stdout, "{json}").is_err() || stdout.flush().is_err() {
                    break;
                }
            }
            Err(e) => {
                let fallback = format!(
                    "{{\"id\":0,\"payload\":{{\"direction\":\"Response\",\"Error\":{{\"message\":\"{}\"}}}}}}",
                    e
                );
                if writeln!(stdout, "{fallback}").is_err() || stdout.flush().is_err() {
                    break;
                }
            }
        }
    }
}

#[derive(Default)]
struct SessionState {
    loaded_spec: Option<String>,
    last_report: Option<CompileReport>,
}

fn handle_request(request: CompilerRequest, state: &mut SessionState) -> CompilerResponse {
    match request {
        CompilerRequest::LoadSpec { source } => {
            state.loaded_spec = Some(source);
            CompilerResponse::Ok
        }
        CompilerRequest::ValidateSpec | CompilerRequest::CompileDryRun => {
            let source = match state.loaded_spec.as_deref() {
                Some(src) => src,
                None => {
                    return CompilerResponse::Error {
                        message: "no spec loaded. send LoadSpec first".to_string(),
                    };
                }
            };
            match compile_from_source(source, "", true) {
                Ok(report) => {
                    let report_json = common_json::to_string(&report).unwrap_or_default();
                    let hash = report.manifest_hash.clone();
                    state.last_report = Some(report);
                    CompilerResponse::CompileReport {
                        manifest_hash: hash,
                        report_json,
                    }
                }
                Err(e) => CompilerResponse::Error {
                    message: e.to_string(),
                },
            }
        }
        CompilerRequest::CompileWrite { out_dir } => {
            let source = match state.loaded_spec.as_deref() {
                Some(src) => src,
                None => {
                    return CompilerResponse::Error {
                        message: "no spec loaded. send LoadSpec first".to_string(),
                    };
                }
            };
            match compile_from_source(source, &out_dir, false) {
                Ok(report) => {
                    let report_json = common_json::to_string(&report).unwrap_or_default();
                    let hash = report.manifest_hash.clone();
                    state.last_report = Some(report);
                    CompilerResponse::CompileReport {
                        manifest_hash: hash,
                        report_json,
                    }
                }
                Err(e) => CompilerResponse::Error {
                    message: e.to_string(),
                },
            }
        }
        CompilerRequest::GetCompileReport => match state.last_report.as_ref() {
            Some(report) => {
                let report_json = common_json::to_string(report).unwrap_or_default();
                CompilerResponse::CompileReport {
                    manifest_hash: report.manifest_hash.clone(),
                    report_json,
                }
            }
            None => CompilerResponse::Error {
                message: "no report available yet".to_string(),
            },
        },
        CompilerRequest::Shutdown => {
            std::process::exit(0);
        }
    }
}

fn compile_pipeline(
    dsl_src: &str,
    out_dir: &str,
    dry_run: bool,
) -> Result<CompileReport, CompilerError> {
    let source = if dsl_src.is_empty() {
        "state Idle {}".to_string()
    } else {
        std::fs::read_to_string(dsl_src).map_err(|e| CompilerError::Io(e.to_string()))?
    };
    compile_from_source(&source, out_dir, dry_run)
}

fn compile_from_source(
    source: &str,
    out_dir: &str,
    dry_run: bool,
) -> Result<CompileReport, CompilerError> {
    let mut parser = Parser::new(source);
    let ast = parser.parse()?;

    let invariant_checker = InvariantChecker;
    invariant_checker.check(&ast)?;

    let determinism_checker = DeterminismChecker;
    determinism_checker.check(&ast)?;

    let runtime_spec = RuntimeSpec::from_ast(&ast);

    let runtime_emitter = RuntimeEmitter::new();
    let runtime_artifacts = runtime_emitter.emit(&runtime_spec);

    let test_emitter = TestEmitter::new();
    let test_artifacts = test_emitter.emit(&runtime_spec);

    let project_emitter = ProjectEmitter::new();
    let project_artifacts = project_emitter.emit(&runtime_spec);

    let artifact_count =
        runtime_artifacts.len() + test_artifacts.len() + project_artifacts.len();

    let manifest_hash =
        output::artifact_manifest::compute_manifest_hash(&runtime_artifacts);
    let manifest = build_manifest(&runtime_artifacts, &manifest_hash);
    let manifest_json = common_json::to_string(&manifest)
        .map_err(|e| CompilerError::Internal(format!("manifest serialization failed: {e}")))?;

    if !dry_run && !out_dir.is_empty() {
        let writer = FsWriter::new(out_dir);
        for (path, content) in &runtime_artifacts {
            writer.write(path, content)?;
        }
        for (path, content) in &test_artifacts {
            writer.write(path, content)?;
        }
        for (path, content) in &project_artifacts {
            writer.write(path, content)?;
        }
        writer.write("artifact_manifest.json", manifest_json.as_bytes())?;
    }

    Ok(CompileReport {
        success: true,
        artifact_count,
        manifest_hash,
        diagnostics: vec![format!("manifest entries: {}", manifest.entries.len())],
    })
}

fn build_manifest(artifacts: &[(String, Vec<u8>)], hash: &str) -> ArtifactManifest {
    let entries = artifacts
        .iter()
        .map(|(path, bytes)| ManifestEntry {
            path: path.clone(),
            size: bytes.len(),
            sha256: sha256_hex(bytes),
        })
        .collect();
    ArtifactManifest {
        entries,
        hash: hash.to_string(),
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

fn print_usage() {
    println!("spec-to-runtime-compiler-backend");
    println!();
    println!("Commands:");
    println!("  compile <dsl-file> [out-dir]   Compile spec DSL and write runtime scaffold");
    println!("  validate <dsl-file>             Validate spec DSL without emitting");
    println!("  dry-run <dsl-file>              Validate + emit in memory only");
    println!("  serve                           Start stdio IPC server mode");
}
