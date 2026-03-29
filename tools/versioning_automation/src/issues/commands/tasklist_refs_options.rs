//! tools/versioning_automation/src/issues/commands/tasklist_refs_options.rs
use crate::issues::{execute::print_non_empty_lines, extract_tasklist_refs};

#[derive(Debug, Clone)]
pub(crate) struct TasklistRefsOptions {
    pub(crate) body: String,
}

impl TasklistRefsOptions {
    pub(crate) fn run_tasklist_refs(self) -> i32 {
        print_non_empty_lines(&extract_tasklist_refs(&self.body).join("\n"));
        0
    }
}
