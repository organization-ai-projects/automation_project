#[allow(dead_code)]
pub fn render_report_screen(generation: u32, hash: &str) -> Vec<String> {
    vec![format!("Report - Generation: {} | Hash: {}", generation, hash)]
}
