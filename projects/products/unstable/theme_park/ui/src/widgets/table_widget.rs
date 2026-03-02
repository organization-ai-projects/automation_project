#![allow(dead_code)]

/// Renders a simple ASCII table from rows of string columns.
pub struct TableWidget {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TableWidget {
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn render(&self) -> String {
        let mut out = self.headers.join(" | ");
        out.push('\n');
        for row in &self.rows {
            out.push_str(&row.join(" | "));
            out.push('\n');
        }
        out
    }
}
