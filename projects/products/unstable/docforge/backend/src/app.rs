use crate::diagnostics::error::Error;
use crate::edit::edit_op::EditOp;
use crate::edit::edit_tx::EditTx;
use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::persistence::doc_snapshot::DocSnapshot;
use crate::persistence::doc_store::DocStore;
use crate::protocol::stdout_writer::StdoutWriter;
use crate::render::html_renderer::HtmlRenderer;
use crate::render::render_target::RenderTarget;
use crate::render::text_renderer::TextRenderer;
use crate::replay::doc_event::DocEvent;
use crate::replay::replay_engine::ReplayEngine;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn run() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Ok(());
    }

    match args[1].as_str() {
        "new" => cmd_new(&args[2..]),
        "edit" => cmd_edit(&args[2..]),
        "render" => cmd_render(&args[2..]),
        "replay" => cmd_replay(&args[2..]),
        _ => Ok(()),
    }
}

fn cmd_new(args: &[String]) -> Result<(), Error> {
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

    let doc = Document::new(DocId::new(uuid_simple()), title);
    let initial_event = DocEvent::new(
        1,
        doc.id.clone(),
        vec![EditOp::SetTitle {
            title: doc.title.clone(),
        }],
    );
    let snapshot = DocSnapshot::create(&doc, 0, vec![initial_event])?;
    let mut store = DocStore::new();
    store.save(snapshot);

    if let Some(path) = out {
        store.save_to_file(&path)?;
    }

    Ok(())
}

fn cmd_edit(args: &[String]) -> Result<(), Error> {
    if args.is_empty() {
        return Err(Error::InvalidOperation(
            "edit requires a file argument".to_string(),
        ));
    }

    let file = &args[0];
    let mut ops_file: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--apply" if i + 1 < args.len() => {
                ops_file = Some(args[i + 1].clone());
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    let mut store = DocStore::load_from_file(file)?;
    if let Some(ops_path) = ops_file {
        let ops_json = std::fs::read_to_string(&ops_path).map_err(|e| Error::Io(e.to_string()))?;
        let ops: Vec<EditOp> = common_json::from_json_str(&ops_json)
            .map_err(|e| Error::Serialization(e.to_string()))?;
        let doc_ids = store.doc_ids();

        for doc_id in doc_ids {
            if let Some(snap) = store.load(&doc_id) {
                let mut doc = snap.document.clone();
                EditTx::from_ops(ops.clone()).apply(&mut doc)?;
                let mut events = snap.events.clone();
                let next_sequence = events.last().map(|event| event.sequence + 1).unwrap_or(1);
                events.push(DocEvent::new(next_sequence, doc.id.clone(), ops.clone()));
                let updated = DocSnapshot::create(&doc, snap.version + 1, events)?;
                store.save(updated);
            }
        }
        store.save_to_file(file)?;
    }

    Ok(())
}

fn cmd_render(args: &[String]) -> Result<(), Error> {
    if args.is_empty() {
        return Err(Error::InvalidOperation(
            "render requires a file argument".to_string(),
        ));
    }

    let file = &args[0];
    let mut target = RenderTarget::Html;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--target" if i + 1 < args.len() => {
                target = match args[i + 1].as_str() {
                    "text" => RenderTarget::Text,
                    _ => RenderTarget::Html,
                };
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    let store = DocStore::load_from_file(file)?;
    let doc_ids = store.doc_ids();
    for doc_id in doc_ids {
        if let Some(snapshot) = store.load(&doc_id) {
            let output = match target {
                RenderTarget::Html => HtmlRenderer::new().render(&snapshot.document),
                RenderTarget::Text => TextRenderer::new().render(&snapshot.document),
            };
            StdoutWriter::write_line(&output);
        }
    }

    Ok(())
}

fn cmd_replay(args: &[String]) -> Result<(), Error> {
    if args.is_empty() {
        return Err(Error::InvalidOperation(
            "replay requires a file argument".to_string(),
        ));
    }

    let file = &args[0];
    let mut target = RenderTarget::Html;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--target" if i + 1 < args.len() => {
                target = match args[i + 1].as_str() {
                    "text" => RenderTarget::Text,
                    _ => RenderTarget::Html,
                };
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    let store = DocStore::load_from_file(file)?;
    for doc_id in store.doc_ids() {
        if let Some(snapshot) = store.load(&doc_id) {
            let mut replayed = Document::new(doc_id.clone(), "Untitled");
            ReplayEngine::new().replay(&mut replayed, &snapshot.events)?;
            let output = match target {
                RenderTarget::Html => HtmlRenderer::new().render(&replayed),
                RenderTarget::Text => TextRenderer::new().render(&replayed),
            };
            StdoutWriter::write_line(&output);
        }
    }

    Ok(())
}

fn uuid_simple() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("doc-{n:016x}")
}
