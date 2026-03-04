// projects/products/unstable/evolutionary_system_generator/ui/src/widgets/table_widget.rs
pub fn render_table(headers: &[&str], rows: &[Vec<String>]) -> Vec<String> {
    let mut out = vec![headers.join(" | ")];
    for row in rows {
        out.push(row.join(" | "));
    }
    out
}
