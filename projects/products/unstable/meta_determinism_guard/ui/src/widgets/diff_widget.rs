pub fn render_diff(original: &str, updated: &str) {
    for line in original.lines() {
        println!("- {}", line);
    }
    for line in updated.lines() {
        println!("+ {}", line);
    }
}
