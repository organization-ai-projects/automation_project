use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactManifest {
    pub name: String,
    pub files: BTreeMap<String, Vec<u8>>,
}

impl ArtifactManifest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            files: BTreeMap::new(),
        }
    }

    pub fn add_file(&mut self, path: impl Into<String>, bytes: Vec<u8>) {
        self.files.insert(path.into(), bytes);
    }

    pub fn sorted_paths(&self) -> Vec<&str> {
        self.files.keys().map(String::as_str).collect()
    }

    pub fn canonical_json(&self) -> Result<String, String> {
        let mut entries: Vec<(String, String)> = Vec::new();
        for (path, bytes) in &self.files {
            entries.push((path.clone(), hash_bytes(bytes)));
        }
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        let mut out = String::new();
        out.push_str("{\"name\":\"");
        out.push_str(&json_escape(&self.name));
        out.push_str("\",\"files\":[");
        for (idx, (path, hash)) in entries.iter().enumerate() {
            if idx > 0 {
                out.push(',');
            }
            out.push_str("[\"");
            out.push_str(&json_escape(path));
            out.push_str("\",\"");
            out.push_str(&json_escape(hash));
            out.push_str("\"]");
        }
        out.push_str("]}");
        Ok(out)
    }
}

fn json_escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            _ => out.push(c),
        }
    }
    out
}

fn hash_bytes(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}
