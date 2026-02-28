// projects/products/unstable/code_forge_engine/ui/src/widgets/table_widget.rs
pub struct TableWidget {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TableWidget {
    pub fn new(headers: Vec<String>) -> Self {
        Self { headers, rows: vec![] }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }
}
