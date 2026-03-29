//! tools/versioning_automation/src/pr/commands/pr_resolve_category_options.rs
use crate::pr::resolve_effective_category;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrResolveCategoryOptions {
    pub(crate) label_category: String,
    pub(crate) title_category: String,
    pub(crate) default_category: String,
}

impl PrResolveCategoryOptions {
    pub(crate) fn run_resolve_category(self) -> i32 {
        let effective = resolve_effective_category(
            &self.label_category,
            &self.title_category,
            &self.default_category,
        );

        println!("{effective}");
        0
    }
}
