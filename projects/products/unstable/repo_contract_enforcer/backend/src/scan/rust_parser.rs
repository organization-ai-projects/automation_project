#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimaryItemViolation {
    pub message: String,
    pub line: Option<u32>,
}

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

    pub fn primary_item_contract_violation(
        file_path: &std::path::Path,
        source: &str,
    ) -> Option<PrimaryItemViolation> {
        let ast = match syn::parse_file(source) {
            Ok(ast) => ast,
            Err(err) => {
                return Some(PrimaryItemViolation {
                    message: format!("rust parse failed: {err}"),
                    line: None,
                });
            }
        };

        let mut primary_items: Vec<String> = Vec::new();
        for item in &ast.items {
            match item {
                syn::Item::Struct(s) => primary_items.push(s.ident.to_string()),
                syn::Item::Enum(e) => primary_items.push(e.ident.to_string()),
                _ => {}
            }
        }

        let stem = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        if primary_items.is_empty() {
            return Some(PrimaryItemViolation {
                message: "file must contain exactly one primary struct or enum".to_string(),
                line: None,
            });
        }
        if primary_items.len() > 1 {
            return Some(PrimaryItemViolation {
                message: "file contains multiple primary struct/enum declarations".to_string(),
                line: None,
            });
        }

        let primary_name = &primary_items[0];
        let expected = to_snake_case(primary_name);
        if expected != stem {
            return Some(PrimaryItemViolation {
                message: format!(
                    "primary item name '{primary_name}' does not match file stem '{stem}'"
                ),
                line: None,
            });
        }

        None
    }
}

fn to_snake_case(input: &str) -> String {
    let mut out = String::new();
    for (idx, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if idx > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}
