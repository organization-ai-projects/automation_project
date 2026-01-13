// projects/products/code_agent_sandbox/src/normalization/normalization_extensions.rs
// Optimize normalize_extension to avoid allocations
pub fn normalize_extension(language: &str) -> &'static str {
    let l = language.trim();
    if l.eq_ignore_ascii_case("rust") || l.eq_ignore_ascii_case("rs") {
        "rs"
    } else if l.eq_ignore_ascii_case("python") || l.eq_ignore_ascii_case("py") {
        "py"
    } else if l.eq_ignore_ascii_case("javascript") || l.eq_ignore_ascii_case("js") {
        "js"
    } else if l.eq_ignore_ascii_case("typescript") || l.eq_ignore_ascii_case("ts") {
        "ts"
    } else if l.eq_ignore_ascii_case("json") {
        "json"
    } else if l.eq_ignore_ascii_case("toml") {
        "toml"
    } else if l.eq_ignore_ascii_case("yaml") || l.eq_ignore_ascii_case("yml") {
        "yml"
    } else if l.eq_ignore_ascii_case("md") || l.eq_ignore_ascii_case("markdown") {
        "md"
    } else {
        "txt"
    }
}
