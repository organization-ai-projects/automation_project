pub fn render_table(headers: &[&str], rows: &[Vec<String>]) {
    let widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let max_row = rows
                .iter()
                .map(|r| r.get(i).map(|s| s.len()).unwrap_or(0))
                .max()
                .unwrap_or(0);
            h.len().max(max_row)
        })
        .collect();

    let header_line: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:width$}", h, width = widths[i]))
        .collect();
    println!("{}", header_line.join(" | "));

    let sep: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
    println!("{}", sep.join("-+-"));

    for row in rows {
        let row_line: Vec<String> = (0..headers.len())
            .map(|i| {
                format!(
                    "{:width$}",
                    row.get(i).map(|s| s.as_str()).unwrap_or(""),
                    width = widths[i]
                )
            })
            .collect();
        println!("{}", row_line.join(" | "));
    }
}
