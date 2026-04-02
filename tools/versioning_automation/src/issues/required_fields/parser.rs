pub(crate) fn trim_whitespace(input: &str) -> String {
    input.trim().to_string()
}

pub(crate) fn extract_field_value(body: &str, field: &str) -> String {
    let field_lc = field.to_lowercase();

    for line in body.lines() {
        let line_trimmed_start = line.trim_start();
        let line_lower = line_trimmed_start.to_lowercase();
        if !line_lower.starts_with(&field_lc) {
            continue;
        }

        let after_field = &line_trimmed_start[field_lc.len()..];
        let after_field = after_field.trim_start();
        if !after_field.starts_with(':') {
            continue;
        }

        return after_field[1..].to_string();
    }

    String::new()
}

pub(crate) fn body_has_section(body: &str, section: &str) -> bool {
    let expected = section.trim().to_lowercase();
    body.lines()
        .any(|line| line.trim().to_lowercase() == expected)
}
