#![cfg(test)]

use crate::edit::edit_op::EditOp;
use crate::edit::edit_tx::EditTx;
use crate::model::block::Block;
use crate::model::block_id::BlockId;
use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::model::inline::Inline;
use crate::replay::doc_event::DocEvent;
use crate::replay::replay_engine::ReplayEngine;

#[test]
fn test_replay_produces_same_document_as_direct_apply() {
    let doc_id = DocId::new("doc-replay");
    let ops = vec![
        EditOp::InsertBlock {
            position: 0,
            block: Block::Heading {
                id: BlockId("h1".to_string()),
                level: 1,
                content: vec![Inline::Text("Hello".to_string())],
                style: None,
            },
        },
        EditOp::SetTitle {
            title: "Replay".to_string(),
        },
    ];

    let mut direct = Document::new(doc_id.clone(), "Initial");
    let applied = EditTx::from_ops(ops.clone()).apply(&mut direct);
    assert!(applied.is_ok());

    let mut replayed = Document::new(doc_id.clone(), "Initial");
    let replayed_result =
        ReplayEngine::new().replay(&mut replayed, &[DocEvent::new(1, doc_id, ops)]);
    assert!(replayed_result.is_ok());

    assert_eq!(direct, replayed);
}

#[test]
fn test_replay_rejects_mismatched_doc_id() {
    let mut doc = Document::new(DocId::new("doc-a"), "Initial");
    let event = DocEvent::new(
        1,
        DocId::new("doc-b"),
        vec![EditOp::SetTitle {
            title: "Updated".to_string(),
        }],
    );

    let result = ReplayEngine::new().replay(&mut doc, &[event]);
    assert!(result.is_err());
}

#[test]
fn test_replay_rejects_non_increasing_sequence() {
    let doc_id = DocId::new("doc-a");
    let mut doc = Document::new(doc_id.clone(), "Initial");
    let first = DocEvent::new(
        2,
        doc_id.clone(),
        vec![EditOp::SetTitle {
            title: "A".to_string(),
        }],
    );
    let second = DocEvent::new(
        2,
        doc_id,
        vec![EditOp::SetTitle {
            title: "B".to_string(),
        }],
    );

    let result = ReplayEngine::new().replay(&mut doc, &[first, second]);
    assert!(result.is_err());
}
