use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Response {
    Ok {
        id: u64,
    },
    Error {
        id: u64,
        code: String,
        message: String,
    },
    Snapshot {
        id: u64,
        hash: String,
        tick: u64,
    },
    Report {
        id: u64,
        run_hash: String,
        report_json: String,
    },
}

impl Response {
    pub fn ok(id: u64) -> Self {
        Self::Ok { id }
    }

    pub fn error(id: u64, code: &str, message: &str) -> Self {
        Self::Error {
            id,
            code: code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn snapshot(id: u64, hash: String, tick: u64) -> Self {
        Self::Snapshot { id, hash, tick }
    }

    pub fn report(id: u64, run_hash: String, report_json: String) -> Self {
        Self::Report {
            id,
            run_hash,
            report_json,
        }
    }
}
