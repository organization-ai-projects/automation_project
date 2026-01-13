// projects/products/code_agent_sandbox/src/journal_line.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct JournalLine<'a, T: Serialize> {
    pub run_id: &'a str,
    pub event: &'a str,
    pub timestamp: &'a str,
    pub payload: &'a T,
}
