// projects/products/unstable/hospital_tycoon/ui/src/widgets/table_widget.rs

pub struct TableWidget {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TableWidget {
    pub fn new(headers: Vec<String>) -> Self {
        Self { headers, rows: Vec::new() }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn render(&self) {
        let col_widths: Vec<usize> = self.headers.iter().enumerate().map(|(i, h)| {
            let max_row = self.rows.iter().map(|r| r.get(i).map(|s| s.len()).unwrap_or(0)).max().unwrap_or(0);
            h.len().max(max_row)
        }).collect();

        let header: String = self.headers.iter().enumerate()
            .map(|(i, h)| format!("{:<width$}", h, width = col_widths[i]))
            .collect::<Vec<_>>()
            .join(" | ");
        println!("{}", header);
        println!("{}", "-".repeat(header.len()));

        for row in &self.rows {
            let line: String = row.iter().enumerate()
                .map(|(i, cell)| format!("{:<width$}", cell, width = col_widths.get(i).copied().unwrap_or(0)))
                .collect::<Vec<_>>()
                .join(" | ");
            println!("{}", line);
        }
    }
}
