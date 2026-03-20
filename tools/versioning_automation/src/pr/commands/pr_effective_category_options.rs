#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrEffectiveCategoryOptions {
    pub(crate) labels_raw: String,
    pub(crate) title: Option<String>,
    pub(crate) title_category: Option<String>,
    pub(crate) default_category: String,
}
