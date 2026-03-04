// projects/products/unstable/protocol_builder/backend/src/schema/schema.rs
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::endpoint_spec::EndpointSpec;
use super::message_spec::MessageSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub messages: Vec<MessageSpec>,
    pub endpoints: Vec<EndpointSpec>,
}

impl Schema {
    /// Returns messages sorted by name for deterministic output.
    pub fn sorted_messages(&self) -> Vec<&MessageSpec> {
        let map: BTreeMap<&str, &MessageSpec> =
            self.messages.iter().map(|m| (m.name.as_str(), m)).collect();
        map.values().copied().collect()
    }

    /// Returns endpoints sorted by name for deterministic output.
    pub fn sorted_endpoints(&self) -> Vec<&EndpointSpec> {
        let map: BTreeMap<&str, &EndpointSpec> = self
            .endpoints
            .iter()
            .map(|e| (e.name.as_str(), e))
            .collect();
        map.values().copied().collect()
    }
}
