// projects/products/unstable/protocol_builder/backend/src/protocol/server_state.rs
use crate::{output, schema};

pub struct ServerState {
    pub schema: Option<schema::Schema>,
    pub manifest: Option<output::ArtifactManifest>,
}
