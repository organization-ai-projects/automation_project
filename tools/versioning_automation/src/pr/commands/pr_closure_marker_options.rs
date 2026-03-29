//! tools/versioning_automation/src/pr/commands/pr_closure_marker_options.rs
use crate::pr::closure_marker::{apply_marker, remove_marker};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrClosureMarkerOptions {
    pub(crate) text: String,
    pub(crate) keyword_pattern: String,
    pub(crate) issue: String,
    pub(crate) mode: String,
}

impl PrClosureMarkerOptions {
    pub(crate) fn run_closure_marker(self) -> i32 {
        let result = match self.mode.as_str() {
            "apply" => apply_marker(&self.text, &self.keyword_pattern, &self.issue),
            "remove" => remove_marker(&self.text, &self.keyword_pattern, &self.issue),
            _ => {
                eprintln!("--mode must be apply or remove");
                return 2;
            }
        };

        match result {
            Ok(text) => {
                print!("{text}");
                0
            }
            Err(err) => {
                eprintln!("{err}");
                2
            }
        }
    }
}
