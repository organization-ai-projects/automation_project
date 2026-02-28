// projects/products/unstable/protocol_builder/backend/src/generate/server_stub_emitter.rs
use crate::schema::ProtocolSchema;

pub struct ServerStubEmitter;

impl ServerStubEmitter {
    /// Emits a deterministic server stub as a pseudocode string.
    pub fn emit(schema: &ProtocolSchema) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "// Server stub for protocol: {} v{}\n",
            schema.name, schema.version
        ));
        out.push_str("pub trait Server {\n");
        for endpoint in schema.sorted_endpoints() {
            out.push_str(&format!(
                "    fn {}(&self, req: {}) -> {};\n",
                endpoint.name, endpoint.request, endpoint.response
            ));
        }
        out.push_str("}\n");
        out
    }
}
