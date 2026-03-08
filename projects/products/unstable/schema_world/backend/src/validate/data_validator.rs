use crate::schemas::schema::Schema;
use crate::schemas::type_spec::TypeSpec;
use common_json::Json;

pub struct DataValidator;

impl DataValidator {
    pub fn validate_record(schema: &Schema, record: &Json) -> Result<(), String> {
        let obj = record
            .as_object()
            .ok_or_else(|| "record must be a JSON object".to_string())?;

        for field in &schema.fields {
            let value = obj.get(&field.name);
            if field.required && value.is_none() {
                return Err(format!("missing required field '{}'", field.name));
            }

            if let Some(v) = value {
                let ok = match field.ty {
                    TypeSpec::Bool => v.as_bool().is_some(),
                    TypeSpec::Number => v.as_f64().is_some(),
                    TypeSpec::String => v.as_str().is_some(),
                    TypeSpec::Json => true,
                };
                if !ok {
                    return Err(format!("field '{}' has invalid type", field.name));
                }
            }
        }

        Ok(())
    }
}
