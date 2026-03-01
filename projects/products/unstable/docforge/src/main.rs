mod diagnostics;
mod edit;
mod layout;
mod model;
mod persistence;
mod public_api;
mod render;
mod replay;

use crate::diagnostics::error::DocError;
use crate::edit::edit_op::EditOp;
use crate::edit::edit_tx::EditTx;
use crate::model::block::Block;
use crate::model::block_id::BlockId;
use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::model::inline::Inline;
use crate::persistence::doc_snapshot::DocSnapshot;
use crate::persistence::doc_store::DocStore;
use crate::render::html_renderer::HtmlRenderer;
use crate::render::text_renderer::TextRenderer;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), DocError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    match args[1].as_str() {
        "new" => cmd_new(&args[2..]),
        "edit" => cmd_edit(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            Ok(())
        }
    }
}

fn print_usage() {
    println!("docforge - document forge CLI");
    println!();
    println!("Usage:");
    println!("  docforge new [--title <title>] [--out <file>]");
    println!("  docforge edit <file> --apply <ops_file> [--target html|text]");
}

fn cmd_new(args: &[String]) -> Result<(), DocError> {
    let mut title = "Untitled".to_string();
    let mut out: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--title" if i + 1 < args.len() => {
                title = args[i + 1].clone();
                i += 2;
            }
            "--out" if i + 1 < args.len() => {
                out = Some(args[i + 1].clone());
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    let doc_id = DocId::new(uuid_simple());
    let doc = Document::new(doc_id, title);
    let snapshot = DocSnapshot::create(&doc, 0, vec![])?;
    let mut store = DocStore::new();
    store.save(snapshot);

    if let Some(path) = out {
        store.save_to_file(&path)?;
        println!("Document saved to {path}");
    } else {
        let json = serde_json::to_string_pretty(&doc)
            .map_err(|e| DocError::Serialization(e.to_string()))?;
        println!("{json}");
    }
    Ok(())
}

fn cmd_edit(args: &[String]) -> Result<(), DocError> {
    if args.is_empty() {
        return Err(DocError::InvalidOperation(
            "edit requires a file argument".into(),
        ));
    }
    let file = &args[0];
    let mut ops_file: Option<String> = None;
    let mut target = "html".to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--apply" if i + 1 < args.len() => {
                ops_file = Some(args[i + 1].clone());
                i += 2;
            }
            "--target" if i + 1 < args.len() => {
                target = args[i + 1].clone();
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    let mut store = DocStore::load_from_file(file)?;

    if let Some(ops_path) = ops_file {
        let ops_json =
            std::fs::read_to_string(&ops_path).map_err(|e| DocError::Io(e.to_string()))?;
        let ops: Vec<EditOp> =
            serde_json::from_str(&ops_json).map_err(|e| DocError::Serialization(e.to_string()))?;

        // We need to find the doc in the store - apply to first doc found
        // Load, apply, save back

        // Reload raw JSON to get doc IDs
        let raw = std::fs::read_to_string(file).map_err(|e| DocError::Io(e.to_string()))?;
        let map: serde_json::Value =
            serde_json::from_str(&raw).map_err(|e| DocError::Serialization(e.to_string()))?;

        if let Some(obj) = map.as_object() {
            for (doc_id_str, _) in obj {
                let doc_id = DocId::new(doc_id_str.clone());
                if let Some(snap) = store.load(&doc_id) {
                    let mut doc = snap.document.clone();
                    let tx = EditTx::from_ops(ops.clone());
                    tx.apply(&mut doc)?;
                    let new_snap = DocSnapshot::create(&doc, snap.version + 1, vec![])?;
                    store.save(new_snap);
                }
            }
        }
        store.save_to_file(file)?;
    }

    // Re-load to render
    let raw = std::fs::read_to_string(file).map_err(|e| DocError::Io(e.to_string()))?;
    let map: serde_json::Value =
        serde_json::from_str(&raw).map_err(|e| DocError::Serialization(e.to_string()))?;

    if let Some(obj) = map.as_object() {
        for (doc_id_str, _) in obj {
            let doc_id = DocId::new(doc_id_str.clone());
            if let Some(snap) = store.load(&doc_id) {
                let doc = &snap.document;
                let output = match target.as_str() {
                    "text" => TextRenderer::new().render(doc),
                    _ => HtmlRenderer::new().render(doc),
                };
                println!("{output}");
            }
        }
    }

    Ok(())
}

fn uuid_simple() -> String {
    // deterministic-enough ID based on process + counter, no system time
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("doc-{n:016x}")
}

#[allow(dead_code)]
fn make_sample_doc() -> Document {
    let doc_id = DocId::new("sample");
    let mut doc = Document::new(doc_id, "Sample Document");
    let mut tx = EditTx::new();
    tx.add_op(EditOp::InsertBlock {
        position: 0,
        block: Block::Paragraph {
            id: BlockId::new("p1"),
            content: vec![Inline::Text("This is a sample paragraph.".into())],
            style: None,
        },
    });
    tx.apply(&mut doc).expect("sample doc creation failed");
    doc
}
