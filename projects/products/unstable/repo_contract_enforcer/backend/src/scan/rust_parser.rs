#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustParser;

impl RustParser {
    pub fn first_line_of(haystack: &str, needle: &str) -> Option<u32> {
        haystack
            .find(needle)
            .map(|idx| haystack[..idx].chars().filter(|c| *c == '\n').count() as u32 + 1)
    }

    pub fn first_line_of_any(haystack: &str, needles: &[&str]) -> Option<u32> {
        let mut best: Option<usize> = None;
        for needle in needles {
            if let Some(idx) = haystack.find(needle) {
                best = Some(match best {
                    Some(current) => current.min(idx),
                    None => idx,
                });
            }
        }
        best.map(|idx| haystack[..idx].chars().filter(|c| *c == '\n').count() as u32 + 1)
    }
}
