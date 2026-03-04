// projects/products/unstable/meta_determinism_guard/ui/src/widgets/diff_widget.rs
pub fn render_diff(original: &str, updated: &str) {
    for line in original.lines() {
        println!("- {line}");
    }
    for line in updated.lines() {
        println!("+ {line}");
    }
}

pub fn render_prefixed_diff(diff_text: &str) {
    let mut original = String::new();
    let mut updated = String::new();

    for line in diff_text.lines() {
        if let Some(value) = line.strip_prefix("- ") {
            if !original.is_empty() {
                original.push('\n');
            }
            original.push_str(value);
        } else if let Some(value) = line.strip_prefix("+ ") {
            if !updated.is_empty() {
                updated.push('\n');
            }
            updated.push_str(value);
        } else {
            println!("{line}");
        }
    }

    if !original.is_empty() || !updated.is_empty() {
        render_diff(&original, &updated);
    }
}
