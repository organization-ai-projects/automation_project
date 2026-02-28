pub fn diff_strings(original: &str, canonical: &str) -> String {
    if original == canonical {
        return String::from("(no diff)");
    }
    format!("- {}\n+ {}", original, canonical)
}
