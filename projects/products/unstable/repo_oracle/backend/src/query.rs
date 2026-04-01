use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Query {
    ReverseDeps { crate_name: String },
    PublicItems { crate_name: String },
    FindSymbol { substring: String },
}
