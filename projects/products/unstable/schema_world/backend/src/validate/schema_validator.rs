use crate::schemas::schema::Schema;
use std::collections::BTreeSet;

pub struct SchemaValidator;

impl SchemaValidator {
    pub fn validate(schema: &Schema) -> Result<(), String> {
        if schema.name.trim().is_empty() {
            return Err("schema name must not be empty".to_string());
        }

        let mut names = BTreeSet::new();
        for field in &schema.fields {
            if field.name.trim().is_empty() {
                return Err("field name must not be empty".to_string());
            }
            if !names.insert(field.name.as_str()) {
                return Err(format!("duplicate field '{}'", field.name));
            }
        }

        Ok(())
    }
}
