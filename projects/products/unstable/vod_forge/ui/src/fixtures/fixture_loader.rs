use crate::app::CatalogEntry;
use crate::diagnostics::UiError;
use common_json::{Json, JsonAccess};

pub struct FixtureLoader;

impl FixtureLoader {
    pub fn load_catalog_json(path: &str) -> Result<Vec<CatalogEntry>, UiError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| UiError::Fixture(format!("read error: {}", e)))?;
        let catalog: Json = common_json::from_json_str(&content)
            .map_err(|e| UiError::Fixture(format!("parse error: {}", e)))?;

        let titles = catalog
            .get_field("titles")
            .and_then(|v| v.as_array_strict())
            .map_err(|e| UiError::Fixture(format!("invalid titles field: {}", e)))?;

        let mut entries = Vec::new();
        for title in titles {
            let id = title
                .get_field("id")
                .and_then(|v| v.as_str_strict())
                .map_err(|e| UiError::Fixture(format!("invalid title id: {}", e)))?;
            let name = title
                .get_field("name")
                .and_then(|v| v.as_str_strict())
                .map_err(|e| UiError::Fixture(format!("invalid title name: {}", e)))?;
            let year = title
                .get_field("year")
                .and_then(|v| v.as_u64_strict())
                .map_err(|e| UiError::Fixture(format!("invalid title year: {}", e)))?;
            let year = u16::try_from(year)
                .map_err(|_| UiError::Fixture("title year out of range".to_string()))?;
            entries.push(CatalogEntry {
                id: id.to_string(),
                name: name.to_string(),
                year,
            });
        }

        Ok(entries)
    }
}
