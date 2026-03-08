#![cfg(test)]

use crate::edit::edit_op::EditOp;
use crate::model::doc_id::DocId;
use crate::replay::doc_event::DocEvent;

#[test]
fn test_doc_event_constructor_sets_fields() {
    let event = DocEvent::new(
        7,
        DocId::new("doc"),
        vec![EditOp::SetTitle {
            title: "A".to_string(),
        }],
    );
    assert_eq!(event.sequence, 7);
    assert_eq!(event.doc_id.0, "doc");
    assert_eq!(event.ops.len(), 1);
}
