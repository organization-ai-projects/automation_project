// projects/products/unstable/protocol_builder/backend/src/parse/schema_parser.rs
use anyhow::Result;

use crate::schema::Schema;

pub struct SchemaParser;

impl SchemaParser {
    pub fn parse_file(path: &str) -> Result<Schema> {
        let content = std::fs::read_to_string(path)?;
        let schema: Schema = common_json::from_json_str(&content)?;
        Ok(schema)
    }
}
