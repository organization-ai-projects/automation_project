use crate::pr::contracts::cli::pr_directives_format::PrDirectivesFormat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrDirectivesOptions {
    pub(crate) text: String,
    pub(crate) format: PrDirectivesFormat,
    pub(crate) unique: bool,
}
