use crate::app::app_state::CatalogEntry;
use crate::diagnostics::UiError;
use serde::Deserialize;

#[derive(Deserialize)]
struct FixtureCatalog {
    titles: Vec<FixtureTitle>,
}

#[derive(Deserialize)]
struct FixtureTitle {
    id: String,
    name: String,
    year: u16,
}

pub struct FixtureLoader;

impl FixtureLoader {
    pub fn load_catalog_json(path: &str) -> Result<Vec<CatalogEntry>, UiError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| UiError::Fixture(format!("read error: {}", e)))?;
        let catalog: FixtureCatalog = common_json::from_json_str(&content)
            .map_err(|e| UiError::Fixture(format!("parse error: {}", e)))?;
        Ok(catalog
            .titles
            .into_iter()
            .map(|t| CatalogEntry {
                id: t.id,
                name: t.name,
                year: t.year,
            })
            .collect())
    }
}
