pub struct TableWidget;

impl TableWidget {
    pub fn render(headers: &[&str], rows: &[Vec<String>]) {
        println!("{}", headers.join("\t"));
        for row in rows {
            println!("{}", row.join("\t"));
        }
    }
}
