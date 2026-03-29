//! tools/versioning_automation/src/issues/commands/extract_refs_options.rs
use std::{collections::HashSet, fs};

use regex::Regex;

use crate::issues::commands::ExtractRefsProfile;

#[derive(Debug, Clone)]
pub(crate) struct ExtractRefsOptions {
    pub(crate) profile: ExtractRefsProfile,
    pub(crate) text: Option<String>,
    pub(crate) file: Option<String>,
}

impl ExtractRefsOptions {
    pub(crate) fn run_extract_refs(self) -> i32 {
        let text = if let Some(raw) = self.text {
            raw
        } else if let Some(path) = self.file {
            match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("failed to read --file '{}': {}", path, err);
                    return 1;
                }
            }
        } else {
            eprintln!("extract-refs requires one input: --text or --file");
            return 1;
        };

        let re = match self.profile {
            ExtractRefsProfile::Hook => Regex::new(
                r"(?i)(cancel[\s_-]*closes|closes|fixes|part\s+of|reopen|reopens)\s+#([0-9]+)",
            ),
            ExtractRefsProfile::Audit => Regex::new(
                r"(?i)(cancel[\s_-]*closes|closes|fixes|resolves|part\s+of|related\s+to|reopen|reopens)\s+#([0-9]+)",
            ),
        };
        let re = match re {
            Ok(value) => value,
            Err(err) => {
                eprintln!("failed to compile issue refs regex: {err}");
                return 1;
            }
        };

        let mut seen = HashSet::<String>::new();
        for caps in re.captures_iter(&text) {
            let Some(action_raw) = caps.get(1).map(|m| m.as_str()) else {
                continue;
            };
            let Some(issue_number) = caps.get(2).map(|m| m.as_str()) else {
                continue;
            };
            let action = action_raw.to_ascii_lowercase();
            let key = format!("{action}|{issue_number}");
            if seen.insert(key) {
                println!("{action}|{issue_number}");
            }
        }

        0
    }
}
