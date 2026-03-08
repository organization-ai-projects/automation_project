#![cfg(test)]

use crate::edit::edit_op::EditOp;
use crate::edit::undo_stack::UndoStack;

#[test]
fn test_undo_stack_push_pop() {
    let mut stack = UndoStack::new();
    assert!(stack.is_empty());

    stack.push(vec![EditOp::SetTitle {
        title: "A".to_string(),
    }]);
    assert_eq!(stack.len(), 1);

    let popped = stack.pop();
    assert!(popped.is_some());
    assert!(stack.is_empty());
}
