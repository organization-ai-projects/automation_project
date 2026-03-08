use crate::replay::doc_event::DocEvent;

pub struct UndoStack {
    history: Vec<DocEvent>,
}

impl UndoStack {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn push(&mut self, event: DocEvent) {
        self.history.push(event);
    }

    pub fn pop(&mut self) -> Option<DocEvent> {
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
