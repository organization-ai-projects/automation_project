// projects/products/unstable/evolutionary_system_generator/ui/src/widgets/plot_widget.rs
pub fn render_plot(values: &[f64]) -> Vec<String> {
    values.iter().map(|v| format!("{:.4}", v)).collect()
}
