pub struct TableWidget;

impl TableWidget {
    pub fn render(headers: &[&str], rows: &[Vec<String>]) -> String {
        let col_count = headers.len();
        let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_count {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }
        let mut out = String::new();
        for (i, h) in headers.iter().enumerate() {
            out.push_str(&format!("{:<width$}", h, width = widths[i] + 2));
        }
        out.push('\n');
        out.push_str(&"-".repeat(widths.iter().sum::<usize>() + col_count * 2));
        out.push('\n');
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_count {
                    out.push_str(&format!("{:<width$}", cell, width = widths[i] + 2));
                }
            }
            out.push('\n');
        }
        out
    }
}
