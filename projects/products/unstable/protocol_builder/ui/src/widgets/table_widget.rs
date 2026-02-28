// projects/products/unstable/protocol_builder/ui/src/widgets/table_widget.rs
use std::collections::BTreeMap;

pub struct TableWidget {
    pub rows: BTreeMap<String, String>,
}

impl TableWidget {
    pub fn new() -> Self {
        Self {
            rows: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.rows.insert(key.into(), value.into());
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.rows {
            out.push_str(&format!("{}: {}\n", k, v));
        }
        out
    }
}

impl Default for TableWidget {
    fn default() -> Self {
        Self::new()
    }
}
