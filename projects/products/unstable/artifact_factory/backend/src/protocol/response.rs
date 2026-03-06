use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Ok,
    Error {
        message: String,
    },
    InputsLoaded {
        total: usize,
        reports: usize,
        replays: usize,
        manifests: usize,
        protocol_schemas: usize,
        unknown: usize,
    },
    AnalysisComplete {
        events: usize,
        protocols: usize,
        nodes: usize,
        edges: usize,
    },
    DocsRendered {
        markdown_bytes: usize,
        svg_bytes: usize,
        html_bytes: usize,
    },
    Bundle {
        hash: String,
        manifest: Vec<String>,
    },
}
