use crate::verify::verify_report::{EntryStatus, VerifyReport};

pub fn render_verify_report(report: &VerifyReport) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"entry_count\": {},\n", report.entry_count));
    out.push_str(&format!(
        "  \"ok\": {},\n",
        if report.ok { "true" } else { "false" }
    ));
    out.push_str("  \"results\": [\n");
    for (i, r) in report.results.iter().enumerate() {
        out.push_str("    {\n");
        out.push_str(&format!("      \"path\": {},\n", json_str(&r.path)));
        match &r.status {
            EntryStatus::Ok => {
                out.push_str("      \"status\": \"ok\"\n");
            }
            EntryStatus::HashMismatch { expected, actual } => {
                out.push_str("      \"status\": \"hash_mismatch\",\n");
                out.push_str(&format!(
                    "      \"expected_hash\": {},\n",
                    json_str(expected)
                ));
                out.push_str(&format!("      \"actual_hash\": {}\n", json_str(actual)));
            }
            EntryStatus::SizeMismatch { expected, actual } => {
                out.push_str("      \"status\": \"size_mismatch\",\n");
                out.push_str(&format!("      \"expected_size\": {expected},\n"));
                out.push_str(&format!("      \"actual_size\": {actual}\n"));
            }
        }
        if i + 1 < report.results.len() {
            out.push_str("    },\n");
        } else {
            out.push_str("    }\n");
        }
    }
    out.push_str("  ]\n");
    out.push('}');
    out
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
