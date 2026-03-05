// projects/products/unstable/simkernel/ui/src/widgets/table_widget.rs
pub struct TableWidget {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
impl TableWidget {
    pub fn render(&self) -> String {
        format!("{} (rows={})", self.headers.join(" | "), self.rows.len())
    }
}
