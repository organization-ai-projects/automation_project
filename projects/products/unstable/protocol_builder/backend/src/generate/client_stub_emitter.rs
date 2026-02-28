// projects/products/unstable/protocol_builder/backend/src/generate/client_stub_emitter.rs
use crate::schema::ProtocolSchema;

pub struct ClientStubEmitter;

impl ClientStubEmitter {
    /// Emits a deterministic client stub as a pseudocode string.
    pub fn emit(schema: &ProtocolSchema) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "// Client stub for protocol: {} v{}\n",
            schema.name, schema.version
        ));
        out.push_str("pub struct Client;\n");
        out.push_str("impl Client {\n");
        for endpoint in schema.sorted_endpoints() {
            out.push_str(&format!(
                "    pub fn {}(&self, req: {}) -> {} {{ unimplemented!() }}\n",
                endpoint.name, endpoint.request, endpoint.response
            ));
        }
        out.push_str("}\n");
        out
    }
}
