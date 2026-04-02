mod canonical_json;
mod crate_graph;
mod crate_node;
mod diagnostics;
mod module_node;
mod policy;
mod policy_engine;
mod policy_result;
mod public_item;
mod query;
mod query_engine;
mod query_result;
mod report;
mod scanner;
mod snapshot;

#[cfg(test)]
mod tests;

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("");

    let result = match command {
        "scan" => handle_scan(&args[2..]),
        "query" => handle_query(&args[2..]),
        _ => {
            eprintln!("Usage: repo_oracle_backend <scan|query> [OPTIONS]");
            eprintln!("  scan  --root <workspace_root>");
            eprintln!("  query --snapshot <file> --query <file> --policy <file>");
            process::exit(2);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn handle_scan(args: &[String]) -> Result<(), diagnostics::Error> {
    let mut root: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "--root" {
            i += 1;
            root = Some(
                args.get(i)
                    .ok_or_else(|| diagnostics::Error::InvalidCli("--root requires a path".into()))?
                    .clone(),
            );
        }
        i += 1;
    }

    let root = root.ok_or_else(|| diagnostics::Error::InvalidCli("--root is required".into()))?;

    let snap = scanner::WorkspaceScanner::scan(&root)?;
    let json = report::ReportGenerator::generate_snapshot_report(&snap)?;
    println!("{json}");
    Ok(())
}

fn handle_query(args: &[String]) -> Result<(), diagnostics::Error> {
    let mut snapshot_path: Option<String> = None;
    let mut query_path: Option<String> = None;
    let mut policy_path: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--snapshot" => {
                i += 1;
                snapshot_path = Some(
                    args.get(i)
                        .ok_or_else(|| {
                            diagnostics::Error::InvalidCli("--snapshot requires a path".into())
                        })?
                        .clone(),
                );
            }
            "--query" => {
                i += 1;
                query_path = Some(
                    args.get(i)
                        .ok_or_else(|| {
                            diagnostics::Error::InvalidCli("--query requires a path".into())
                        })?
                        .clone(),
                );
            }
            "--policy" => {
                i += 1;
                policy_path = Some(
                    args.get(i)
                        .ok_or_else(|| {
                            diagnostics::Error::InvalidCli("--policy requires a path".into())
                        })?
                        .clone(),
                );
            }
            _ => {}
        }
        i += 1;
    }

    let snapshot_path = snapshot_path
        .ok_or_else(|| diagnostics::Error::InvalidCli("--snapshot is required".into()))?;

    let snap_str = std::fs::read_to_string(&snapshot_path)
        .map_err(|e| diagnostics::Error::Io(e.to_string()))?;
    let snap: snapshot::Snapshot =
        common_json::from_str(&snap_str).map_err(|e| diagnostics::Error::Parse(e.to_string()))?;

    if let Some(qp) = query_path {
        let q_str =
            std::fs::read_to_string(&qp).map_err(|e| diagnostics::Error::Io(e.to_string()))?;
        let queries: Vec<query::Query> =
            common_json::from_str(&q_str).map_err(|e| diagnostics::Error::Parse(e.to_string()))?;
        for q in &queries {
            let result = query_engine::QueryEngine::execute(&snap, q)?;
            let json = report::ReportGenerator::generate_query_report(&result)?;
            println!("{json}");
        }
    }

    if let Some(pp) = policy_path {
        let p_str =
            std::fs::read_to_string(&pp).map_err(|e| diagnostics::Error::Io(e.to_string()))?;
        let pol: policy::Policy =
            common_json::from_str(&p_str).map_err(|e| diagnostics::Error::Parse(e.to_string()))?;
        let result = policy_engine::PolicyEngine::evaluate(&snap, &pol)?;
        let json = report::ReportGenerator::generate_policy_report(&result)?;
        println!("{json}");
    }

    Ok(())
}
