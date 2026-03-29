//! tools/versioning_automation/src/pr/commands/pr_breaking_detect_options.rs
use crate::pr::{breaking_detect::labels_indicate_breaking, text_indicates_breaking};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrBreakingDetectOptions {
    pub(crate) text: String,
    pub(crate) labels_raw: Option<String>,
}

impl PrBreakingDetectOptions {
    pub(crate) fn run_breaking_detect(self) -> i32 {
        let detected = self
            .labels_raw
            .as_deref()
            .is_some_and(labels_indicate_breaking)
            || (!self.text.is_empty() && text_indicates_breaking(&self.text));

        if detected {
            println!("true");
        } else {
            println!("false");
        }

        0
    }
}
