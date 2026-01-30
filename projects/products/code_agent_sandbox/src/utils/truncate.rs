// projects/products/code_agent_sandbox/src/utils/truncate.rs
pub(crate) fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut cut = max;
    while cut > 0 && !s.is_char_boundary(cut) {
        cut -= 1;
    }
    let mut t = s[..cut].to_string();
    t.push_str("\n...[truncated]...");
    t
}
