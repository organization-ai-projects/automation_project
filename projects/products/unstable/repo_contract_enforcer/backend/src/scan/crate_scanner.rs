#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateScanner;

impl CrateScanner {
    pub fn discover_child_crates(product_dir: &std::path::Path) -> Vec<String> {
        let mut crates = Vec::new();
        let read_dir = match std::fs::read_dir(product_dir) {
            Ok(rd) => rd,
            Err(_) => return crates,
        };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if path.join("Cargo.toml").exists()
                && let Some(name) = path.file_name().and_then(|n| n.to_str())
            {
                crates.push(name.to_string());
            }
        }
        crates.sort();
        crates
    }

    pub fn extract_workspace_members(cargo_toml: &str) -> Vec<String> {
        let members_pos = match cargo_toml.find("members") {
            Some(p) => p,
            None => return Vec::new(),
        };
        let after_members = &cargo_toml[members_pos..];
        let open = match after_members.find('[') {
            Some(p) => p,
            None => return Vec::new(),
        };
        let close = match after_members[open + 1..].find(']') {
            Some(p) => open + 1 + p,
            None => return Vec::new(),
        };
        let slice = &after_members[open + 1..close];
        let mut members = Vec::new();
        let mut in_quote = false;
        let mut current = String::new();
        for ch in slice.chars() {
            if ch == '"' {
                if in_quote {
                    members.push(current.clone());
                    current.clear();
                    in_quote = false;
                } else {
                    in_quote = true;
                }
                continue;
            }
            if in_quote {
                current.push(ch);
            }
        }
        members.sort();
        members
    }
}
