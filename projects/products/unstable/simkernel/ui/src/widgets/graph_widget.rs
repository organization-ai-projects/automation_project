// projects/products/unstable/simkernel/ui/src/widgets/graph_widget.rs
pub struct GraphWidget {
    pub values: Vec<i64>,
    pub label: String,
}
impl GraphWidget {
    pub fn render(&self) -> String {
        format!("{}: {:?}", self.label, self.values)
    }
}
