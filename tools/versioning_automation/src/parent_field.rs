use regex::Regex;

pub(crate) fn extract_parent_field(body: &str) -> Option<String> {
    let re =
        Regex::new(r"(?i)^\s*Parent:\s*(#?[0-9]+|none|base|epic|\(none\)|\(base\)|\(epic\))\s*$")
            .ok()?;
    let mut parent_value: Option<String> = None;

    for line in body.lines() {
        if let Some(captures) = re.captures(line) {
            parent_value = captures.get(1).map(|m| m.as_str().trim().to_lowercase());
        }
    }

    parent_value.map(|raw| {
        raw.trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .to_string()
    })
}
