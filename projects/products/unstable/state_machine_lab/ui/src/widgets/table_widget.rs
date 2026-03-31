pub struct TableWidget;

impl TableWidget {
    pub fn render(headers: &[&str], rows: &[Vec<String>]) {
        println!("{}", headers.join(" | "));
        for row in rows {
            println!("{}", row.join(" | "));
        }
    }
}
