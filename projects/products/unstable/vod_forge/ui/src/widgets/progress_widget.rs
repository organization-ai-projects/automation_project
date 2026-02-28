pub struct ProgressWidget;

impl ProgressWidget {
    pub fn render(pct: f32, width: usize) -> String {
        let filled = ((pct / 100.0) * width as f32).round() as usize;
        let filled = filled.min(width);
        let empty = width - filled;
        format!("[{}{}] {:.1}%", "#".repeat(filled), "-".repeat(empty), pct)
    }
}
