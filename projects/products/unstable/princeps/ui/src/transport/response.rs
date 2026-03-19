use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Response {
    RunAccepted { summary: String },
    ReplayLoaded { summary: String },
    ExportReady { output: String },
    Error { message: String },
}
