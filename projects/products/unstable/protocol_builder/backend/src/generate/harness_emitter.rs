// projects/products/unstable/protocol_builder/backend/src/generate/harness_emitter.rs
use crate::schema::ProtocolSchema;

pub struct HarnessEmitter;

impl HarnessEmitter {
    /// Emits a deterministic golden-transcript harness as a pseudocode string.
    pub fn emit(schema: &ProtocolSchema) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "// Harness for protocol: {} v{}\n",
            schema.name, schema.version
        ));
        out.push_str("fn main() {\n");
        for endpoint in schema.sorted_endpoints() {
            out.push_str(&format!(
                "    // test endpoint: {}\n    let _req: {} = Default::default();\n    let _resp: {} = client.{}(_req);\n",
                endpoint.name, endpoint.request, endpoint.response, endpoint.name
            ));
        }
        out.push_str("}\n");
        out
    }
}
