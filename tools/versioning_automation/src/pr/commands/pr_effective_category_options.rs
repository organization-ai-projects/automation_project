//! tools/versioning_automation/src/pr/commands/pr_effective_category_options.rs
use crate::pr::{
    issue_category_from_labels, resolve_category::issue_category_from_title,
    resolve_effective_category,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrEffectiveCategoryOptions {
    pub(crate) labels_raw: String,
    pub(crate) title: Option<String>,
    pub(crate) title_category: Option<String>,
    pub(crate) default_category: String,
}

impl PrEffectiveCategoryOptions {
    pub(crate) fn run_effective_category(self) -> i32 {
        let label_category = issue_category_from_labels(&self.labels_raw);
        let title_category = if let Some(title) = &self.title {
            issue_category_from_title(title)
        } else if let Some(title_category) = &self.title_category {
            title_category.as_str()
        } else {
            "Unknown"
        };
        let effective =
            resolve_effective_category(label_category, title_category, &self.default_category);
        println!("{effective}");
        0
    }
}
