// projects/products/unstable/simulation_compiler/backend/src/main.rs
mod diagnostics;
mod dsl;
mod generate;
mod io;
mod model;
mod output;
mod protocol;
mod public_api;
mod validate;

use diagnostics::backend_error::CompilerError;
use dsl::parser::Parser;
use generate::fixture_emitter::FixtureEmitter;
use generate::golden_emitter::GoldenEmitter;
use generate::pack_emitter::PackEmitter;
use io::fs_writer::FsWriter;
use model::pack_spec::PackSpec;
use output::artifact_manifest::{ArtifactManifest, ManifestEntry};
use output::compile_report::CompileReport;
use protocol::message::{IpcMessage, IpcPayload};
use protocol::request::CompilerRequest;
use protocol::response::CompilerResponse;
use sha2::{Digest, Sha256};
use validate::determinism_rules::DeterminismRules;
use validate::spec_validator::SpecValidator;

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
    trace_request(CompilerRequest::CompileWrite {
        out_dir: out_dir.clone(),
    });

    tracing::info!(dsl = %dsl_src, out = %out_dir, "compile started");

    match compile_pipeline(&dsl_src, &out_dir, false) {
        Ok(report) => {
            trace_request(CompilerRequest::GetReport);
            trace_response(CompilerResponse::Report {
                json: common_json::to_string(&report).unwrap_or_default(),
            });
            let json = common_json::to_string(&report).unwrap_or_default();
            println!("{json}");
        }
        Err(e) => {
            trace_response(CompilerResponse::Error {
                message: e.to_string(),
            });
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn run_validate(args: &[String]) {
    let dsl_src = args.get(2).cloned().unwrap_or_default();
    trace_request(CompilerRequest::Validate);
    tracing::info!(dsl = %dsl_src, "validate started");

    match compile_pipeline(&dsl_src, "", true) {
        Ok(report) => {
            trace_request(CompilerRequest::GetReport);
            trace_response(CompilerResponse::Report {
                json: common_json::to_string(&report).unwrap_or_default(),
            });
            let json = common_json::to_string(&report).unwrap_or_default();
            println!("{json}");
        }
        Err(e) => {
            trace_response(CompilerResponse::Error {
                message: e.to_string(),
            });
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn run_dry_run(args: &[String]) {
    let dsl_src = args.get(2).cloned().unwrap_or_default();
    trace_request(CompilerRequest::CompileDryRun);
    tracing::info!(dsl = %dsl_src, "dry-run started");

    match compile_pipeline(&dsl_src, "", true) {
        Ok(report) => {
            trace_response(CompilerResponse::Ok);
            tracing::info!(
                artifacts = report.artifact_count,
                manifest_hash = %report.manifest_hash,
                "dry-run complete"
            );
        }
        Err(e) => {
            trace_response(CompilerResponse::Error {
                message: e.to_string(),
            });
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn compile_pipeline(
    dsl_src: &str,
    out_dir: &str,
    dry_run: bool,
) -> Result<CompileReport, CompilerError> {
    let source = if dsl_src.is_empty() {
        // Minimal inline stub for demo/bootstrap.
        "component Sensor { field: u32 }".to_string()
    } else {
        std::fs::read_to_string(dsl_src).map_err(|e| CompilerError::Io(e.to_string()))?
    };
    trace_request(CompilerRequest::LoadDsl {
        source: source.clone(),
    });

    let mut parser = Parser::new(&source);
    let ast = parser.parse()?;

    let validator = SpecValidator::new();
    validator.validate(&ast)?;

    let det_rules = DeterminismRules;
    det_rules.check(&ast)?;

    let pack_spec = PackSpec::from_ast(&ast);

    let emitter = PackEmitter::new();
    let artifacts = emitter.emit(&pack_spec);

    let fixture_emitter = FixtureEmitter::new();
    let fixtures = fixture_emitter.emit(&pack_spec);

    let golden_emitter = GoldenEmitter::new();
    let goldens = golden_emitter.emit(&pack_spec);

    let artifact_count = artifacts.len() + fixtures.len() + goldens.len();

    let manifest_hash = output::manifest_hash::compute_hash(&artifacts);
    let manifest = build_manifest(&artifacts, &manifest_hash);
    let manifest_json = common_json::to_string(&manifest)
        .map_err(|e| CompilerError::Internal(format!("manifest serialization failed: {e}")))?;

    if !dry_run && !out_dir.is_empty() {
        let writer = FsWriter::new(out_dir);
        for (path, content) in &artifacts {
            writer.write(path, content)?;
        }
        for (path, content) in &fixtures {
            writer.write(path, content)?;
        }
        for (path, content) in &goldens {
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

fn trace_request(request: CompilerRequest) {
    let message = IpcMessage {
        id: 1,
        payload: IpcPayload::Request(request),
    };
    if let Ok(json) = io::json_codec::encode(&message) {
        let _: Result<IpcMessage, _> = io::json_codec::decode(&json);
    }
}

fn trace_response(response: CompilerResponse) {
    let message = IpcMessage {
        id: 1,
        payload: IpcPayload::Response(response),
    };
    if let Ok(json) = io::json_codec::encode(&message) {
        let _: Result<IpcMessage, _> = io::json_codec::decode(&json);
    }
}

fn print_usage() {
    println!("simulation-compiler-backend");
    println!();
    println!("Commands:");
    println!("  compile <dsl-file> [out-dir]   Compile DSL and write pack scaffold");
    println!("  validate <dsl-file>             Validate DSL without emitting");
    println!("  dry-run <dsl-file>              Validate + emit in memory only");
}
