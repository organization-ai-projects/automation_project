#[derive(Debug, Clone)]
pub(crate) struct RequiredFieldsValidateOptions {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) labels_raw: String,
    pub(crate) mode: RequiredFieldsValidationMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RequiredFieldsValidationMode {
    Title,
    Body,
    Content,
}
