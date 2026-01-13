pub fn normalize_rel(rel: &str) -> String {
    let mut s = rel.trim().replace('\\', "/");
    while s.starts_with("./") {
        s = s[2..].to_string();
    }
    while s.starts_with('/') {
        s = s[1..].to_string();
    }
    s
}
