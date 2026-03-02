pub struct TableWidget;

impl TableWidget {
    pub fn render(headers: &[&str], rows: &[Vec<String>]) {
        let col_widths: Vec<usize> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                rows.iter()
                    .map(|r| r.get(i).map(|s| s.len()).unwrap_or(0))
                    .max()
                    .unwrap_or(0)
                    .max(h.len())
            })
            .collect();

        // Header row
        let header_row: Vec<String> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:width$}", h, width = col_widths[i]))
            .collect();
        println!("| {} |", header_row.join(" | "));

        // Separator
        let sep: Vec<String> = col_widths.iter().map(|&w| "-".repeat(w)).collect();
        println!("|-{}-|", sep.join("-|-"));

        // Data rows
        for row in rows {
            let cells: Vec<String> = col_widths
                .iter()
                .enumerate()
                .map(|(i, &w)| {
                    let cell = row.get(i).map(String::as_str).unwrap_or("");
                    format!("{:width$}", cell, width = w)
                })
                .collect();
            println!("| {} |", cells.join(" | "));
        }
    }
}
