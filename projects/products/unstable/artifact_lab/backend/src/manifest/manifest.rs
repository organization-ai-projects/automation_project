use crate::diagnostics::Error;
use crate::manifest::ManifestEntry;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Manifest {
    pub entries: Vec<ManifestEntry>,
    pub format_version: String,
}

impl Manifest {
    pub fn new(mut entries: Vec<ManifestEntry>) -> Self {
        entries.sort_by(|a, b| a.path.cmp(&b.path));
        Self {
            entries,
            format_version: "1".to_string(),
        }
    }

    pub fn to_canonical_json(&self) -> String {
        let mut out = String::new();
        out.push_str("{\n");
        out.push_str("  \"entries\": [\n");
        for (i, entry) in self.entries.iter().enumerate() {
            out.push_str("    {\n");
            out.push_str(&format!("      \"hash\": {},\n", json_str(&entry.hash)));
            out.push_str(&format!("      \"path\": {},\n", json_str(&entry.path)));
            out.push_str(&format!("      \"size\": {}\n", entry.size));
            if i + 1 < self.entries.len() {
                out.push_str("    },\n");
            } else {
                out.push_str("    }\n");
            }
        }
        out.push_str("  ],\n");
        out.push_str(&format!(
            "  \"format_version\": {}\n",
            json_str(&self.format_version)
        ));
        out.push('}');
        out
    }

    pub fn from_canonical_json(json: &str) -> Result<Self, Error> {
        common_json::from_str(json).map_err(|e| Error::ManifestFormat(e.to_string()))
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch <= '\u{1F}' => {
                use std::fmt::Write;
                let _ = write!(out, "\\u{:04x}", ch as u32);
            }
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}
