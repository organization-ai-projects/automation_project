use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct TableWidget {
    rows: BTreeMap<String, String>,
}

impl TableWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: &str, value: String) {
        self.rows.insert(key.to_string(), value);
    }

    pub fn render(&self) -> String {
        self.rows
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
