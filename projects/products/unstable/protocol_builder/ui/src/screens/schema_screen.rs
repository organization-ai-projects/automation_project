// projects/products/unstable/protocol_builder/ui/src/screens/schema_screen.rs

#[derive(Debug, Clone, Default)]
pub struct SchemaScreen {
    pub schema_path: String,
}

impl SchemaScreen {
    pub fn render(&self) -> String {
        format!("Schema: {}", self.schema_path)
    }
}
