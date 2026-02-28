#[allow(dead_code)]
pub fn render_plot(values: &[f64]) -> Vec<String> {
    values.iter().map(|v| format!("{:.4}", v)).collect()
}
