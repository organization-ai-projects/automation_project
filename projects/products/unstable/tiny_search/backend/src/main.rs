mod diagnostics;
mod index;
mod persistence;
mod query;
mod rank;
mod tokenize;

#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "index" => {
            if let Err(e) = run_index(&args[2..]) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        "query" => {
            if let Err(e) = run_query(&args[2..]) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  tiny_search index --root <dir> --out <snapshot>");
    eprintln!("  tiny_search query --snapshot <snapshot> --q \"text\" --json");
}

fn run_index(args: &[String]) -> Result<(), diagnostics::error::Error> {
    let mut root: Option<&str> = None;
    let mut out: Option<&str> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--root" => {
                i += 1;
                root = args.get(i).map(|s| s.as_str());
            }
            "--out" => {
                i += 1;
                out = args.get(i).map(|s| s.as_str());
            }
            _ => {}
        }
        i += 1;
    }

    let root = root.ok_or_else(|| {
        diagnostics::error::Error::InvalidArgument("--root is required".to_string())
    })?;
    let out = out.ok_or_else(|| {
        diagnostics::error::Error::InvalidArgument("--out is required".to_string())
    })?;

    let root_path = std::path::Path::new(root);
    let out_path = std::path::Path::new(out);

    let idx = index::index_store::IndexStore::build_from_dir(root_path)?;
    persistence::snapshot_codec::SnapshotCodec::save(&idx, out_path)?;

    eprintln!(
        "Indexed {} documents, snapshot written to {}",
        idx.doc_count,
        out_path.display()
    );
    Ok(())
}

fn run_query(args: &[String]) -> Result<(), diagnostics::error::Error> {
    let mut snapshot: Option<&str> = None;
    let mut query_text: Option<&str> = None;
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--snapshot" => {
                i += 1;
                snapshot = args.get(i).map(|s| s.as_str());
            }
            "--q" => {
                i += 1;
                query_text = args.get(i).map(|s| s.as_str());
            }
            "--json" => {
                json_output = true;
            }
            _ => {}
        }
        i += 1;
    }

    let snapshot_path = snapshot.ok_or_else(|| {
        diagnostics::error::Error::InvalidArgument("--snapshot is required".to_string())
    })?;
    let query_text = query_text.ok_or_else(|| {
        diagnostics::error::Error::InvalidArgument("--q is required".to_string())
    })?;

    let idx =
        persistence::snapshot_codec::SnapshotCodec::load(std::path::Path::new(snapshot_path))?;
    let parsed = query::query_parser::QueryParser::parse(query_text);
    let report = query::query_engine::QueryEngine::execute(&idx, &parsed);

    if json_output {
        let json = common_json::to_string(&report)
            .map_err(|e| diagnostics::error::Error::Serialization(e.to_string()))?;
        println!("{json}");
    } else {
        println!("Query: {:?}", report.query_terms);
        for entry in &report.results {
            println!("  {} (score: {:.6})", entry.doc_id, entry.score);
        }
    }

    Ok(())
}
