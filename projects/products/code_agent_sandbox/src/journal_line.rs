// projects/products/code_agent_sandbox/src/journal_line.rs
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct JournalLine<'a, T: Serialize> {
    pub(crate) run_id: &'a str,
    pub(crate) event: &'a str,
    pub(crate) timestamp: &'a str,
    pub(crate) payload: &'a T,
}
