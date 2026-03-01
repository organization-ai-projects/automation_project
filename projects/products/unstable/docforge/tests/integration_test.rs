use docforge::edit::edit_op::EditOp;
use docforge::edit::edit_tx::EditTx;
use docforge::model::block::Block;
use docforge::model::block_id::BlockId;
use docforge::model::doc_id::DocId;
use docforge::model::document::Document;
use docforge::model::inline::Inline;
use docforge::persistence::doc_snapshot::DocSnapshot;
use docforge::persistence::doc_store::DocStore;
use docforge::render::html_renderer::HtmlRenderer;
use docforge::replay::doc_event::DocEvent;
use docforge::replay::replay_engine::ReplayEngine;

fn make_base_doc() -> Document {
    Document::new(DocId::new("test-doc"), "Integration Test Doc")
}

fn sample_ops() -> Vec<EditOp> {
    vec![
        EditOp::InsertBlock {
            position: 0,
            block: Block::Paragraph {
                id: BlockId::new("p1"),
                content: vec![Inline::Text("Hello, docforge!".into())],
                style: None,
            },
        },
        EditOp::InsertBlock {
            position: 1,
            block: Block::Heading {
                id: BlockId::new("h1"),
                level: 1,
                content: vec![Inline::Text("Introduction".into())],
                style: None,
            },
        },
        EditOp::SetTitle { title: "My Document".into() },
    ]
}

#[test]
fn test_create_apply_snapshot_reload_render() {
    let mut doc = make_base_doc();
    let ops = sample_ops();
    let tx = EditTx::from_ops(ops.clone());
    tx.apply(&mut doc).expect("apply failed");

    let snapshot = DocSnapshot::create(&doc, 1, vec![]).expect("snapshot failed");
    snapshot.verify().expect("checksum mismatch");

    let mut store = DocStore::new();
    store.save(snapshot.clone());

    // Save and reload from file
    let path = "/tmp/docforge_integration_test.json";
    store.save_to_file(path).expect("save_to_file failed");
    let store2 = DocStore::load_from_file(path).expect("load_from_file failed");

    let reloaded = store2.load(&doc.id).expect("doc not found in store");
    reloaded.verify().expect("reloaded checksum mismatch");

    assert_eq!(reloaded.document, doc);

    let renderer = HtmlRenderer::new();
    let html1 = renderer.render(&doc);
    let html2 = renderer.render(&reloaded.document);
    assert_eq!(html1, html2);
    assert!(html1.contains("Hello, docforge!"));
    assert!(html1.contains("Introduction"));
}

#[test]
fn test_replay_event_stream_identical_doc_and_render() {
    let doc_id = DocId::new("replay-doc");
    let ops = sample_ops();

    // Build doc by direct apply
    let mut expected_doc = Document::new(doc_id.clone(), "Integration Test Doc");
    let tx = EditTx::from_ops(ops.clone());
    tx.apply(&mut expected_doc).expect("apply failed");

    // Build doc via replay
    let events = vec![DocEvent::new(1, doc_id.clone(), ops)];
    let mut replayed_doc = Document::new(doc_id.clone(), "Integration Test Doc");
    let engine = ReplayEngine::new();
    engine.replay(&mut replayed_doc, &events).expect("replay failed");

    assert_eq!(expected_doc, replayed_doc);

    let renderer = HtmlRenderer::new();
    let html_expected = renderer.render(&expected_doc);
    let html_replayed = renderer.render(&replayed_doc);
    assert_eq!(html_expected, html_replayed);
}
