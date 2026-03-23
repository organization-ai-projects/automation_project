//! projects/products/unstable/autonomous_dev_ai/backend/src/pr_flow/extract.rs
pub(crate) fn extract_pr_number_from_text(text: &str) -> Option<u64> {
    // Preferred pattern from GitHub URLs: .../pull/<number>
    if let Some((_, suffix)) = text.rsplit_once("/pull/") {
        let digits: String = suffix.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(parsed) = digits.parse::<u64>() {
            return Some(parsed);
        }
    }

    // Fallback pattern: any "#<number>" token in plain output.
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'#' {
            let start = i + 1;
            let mut end = start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > start
                && let Ok(parsed) = text[start..end].parse::<u64>()
            {
                return Some(parsed);
            }
        }
        i += 1;
    }
    None
}
