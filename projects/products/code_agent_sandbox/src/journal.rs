// projects/products/code_agent_sandbox/src/journal.rs
use std::{fs::OpenOptions, io::Write, path::PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::{
    actions::{Action, ActionResult},
    journal_line::JournalLine,
};

pub struct Journal {
    pub file: std::fs::File,
}

impl Journal {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { file })
    }

    pub fn record_action(&mut self, run_id: &str, action: &Action, timestamp: &str) -> Result<()> {
        self.write_line(&JournalLine {
            run_id,
            event: "action",
            timestamp,
            payload: action,
        })
    }

    pub fn record_result(
        &mut self,
        run_id: &str,
        result: &ActionResult,
        timestamp: &str,
    ) -> Result<()> {
        self.write_line(&JournalLine {
            run_id,
            event: "result",
            timestamp,
            payload: result,
        })
    }

    fn write_line<T: Serialize>(&mut self, line: &T) -> Result<()> {
        let s = protocol::to_json_string(line)?;
        writeln!(self.file, "{}", s)?;
        Ok(())
    }
}

impl Default for Journal {
    fn default() -> Self {
        let temp_file = std::fs::File::create("/tmp/journal_default.log")
            .expect("Failed to create default journal file");
        Journal { file: temp_file }
    }
}
