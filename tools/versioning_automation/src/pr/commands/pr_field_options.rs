use crate::pr::commands::pr_field_name::PrFieldName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrFieldOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
    pub(crate) name: PrFieldName,
}
