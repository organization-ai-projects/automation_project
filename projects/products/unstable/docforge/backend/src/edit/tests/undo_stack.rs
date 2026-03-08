#![cfg(test)]

use crate::edit::edit_op::EditOp;
use crate::edit::undo_stack::UndoStack;
use crate::model::doc_id::DocId;
use crate::replay::doc_event::DocEvent;

#[test]
fn test_undo_stack_push_pop() {
    let mut stack = UndoStack::new();
    assert!(stack.is_empty());

    stack.push(DocEvent::new(
        1,
        DocId::new("doc"),
        vec![EditOp::SetTitle {
            title: "A".to_string(),
        }],
    ));
    assert_eq!(stack.len(), 1);

    let popped = stack.pop();
    assert!(popped.is_some());
    assert!(stack.is_empty());
}
