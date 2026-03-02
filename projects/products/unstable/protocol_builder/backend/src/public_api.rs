// projects/products/unstable/protocol_builder/backend/src/public_api.rs
use crate::generate::{ClientStubEmitter, HarnessEmitter, ServerStubEmitter, ValidatorEmitter};
use crate::output::{ArtifactManifest, GenerateReport};
use crate::schema::ProtocolSchema;

/// Builds the artifact manifest from the schema (deterministic).
pub fn build_manifest(schema: &ProtocolSchema) -> ArtifactManifest {
    let mut manifest = ArtifactManifest::new();
    manifest.insert("client_stub.rs", ClientStubEmitter::emit(schema));
    manifest.insert("harness.rs", HarnessEmitter::emit(schema));
    manifest.insert("server_stub.rs", ServerStubEmitter::emit(schema));
    manifest.insert("validator.rs", ValidatorEmitter::emit(schema));
    manifest
}

/// Validates the schema for structural consistency.
pub fn validate_schema(schema: &ProtocolSchema) -> Result<(), String> {
    let msg_names: std::collections::BTreeSet<&str> =
        schema.messages.iter().map(|m| m.name.as_str()).collect();
    for ep in &schema.endpoints {
        if !msg_names.contains(ep.request.as_str()) {
            return Err(format!(
                "endpoint '{}' references unknown request message '{}'",
                ep.name, ep.request
            ));
        }
        if !msg_names.contains(ep.response.as_str()) {
            return Err(format!(
                "endpoint '{}' references unknown response message '{}'",
                ep.name, ep.response
            ));
        }
    }
    Ok(())
}

/// Builds a GenerateReport from the manifest.
pub fn build_report(manifest: &ArtifactManifest) -> GenerateReport {
    GenerateReport::from_manifest(manifest)
}
