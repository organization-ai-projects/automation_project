// projects/products/unstable/protocol_builder/backend/src/generate/validator_emitter.rs
use crate::schema::ProtocolSchema;

pub struct ValidatorEmitter;

impl ValidatorEmitter {
    /// Emits a deterministic validator stub as a pseudocode string.
    pub fn emit(schema: &ProtocolSchema) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "// Validator for protocol: {} v{}\n",
            schema.name, schema.version
        ));
        for msg in schema.sorted_messages() {
            out.push_str(&format!(
                "pub fn validate_{}(msg: &{}) -> bool {{\n",
                msg.name, msg.name
            ));
            for field in &msg.fields {
                out.push_str(&format!("    let _ = &msg.{};\n", field.name));
            }
            out.push_str("    true\n}\n");
        }
        out
    }
}
