// projects/products/unstable/simkernel/ui/src/widgets/log_widget.rs
pub struct LogWidget {
    pub entries: Vec<String>,
}
impl LogWidget {
    pub fn render(&self) -> String {
        self.entries.join("\n")
    }
}
