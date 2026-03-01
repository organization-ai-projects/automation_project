use crate::edit::edit_op::EditOp;

pub struct UndoStack {
    history: Vec<Vec<EditOp>>,
}

impl UndoStack {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn push(&mut self, ops: Vec<EditOp>) {
        self.history.push(ops);
    }

    pub fn pop(&mut self) -> Option<Vec<EditOp>> {
        self.history.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }
}

impl Default for UndoStack {
    fn default() -> Self {
        Self::new()
    }
}
