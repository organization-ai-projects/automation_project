#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrBreakingDetectOptions {
    pub(crate) text: Option<String>,
    pub(crate) labels_raw: Option<String>,
}
