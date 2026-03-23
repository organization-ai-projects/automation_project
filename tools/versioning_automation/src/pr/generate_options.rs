#[derive(Debug, Clone)]
pub(crate) struct GenerateOptions {
    pub(crate) help: bool,
    pub(crate) dry_run: bool,
    pub(crate) main_pr_number: Option<String>,
    pub(crate) create_pr: bool,
    pub(crate) allow_partial_create: bool,
    pub(crate) assume_yes: bool,
    pub(crate) base_ref: Option<String>,
    pub(crate) head_ref: Option<String>,
    pub(crate) duplicate_mode: Option<String>,
    pub(crate) auto_edit_pr_number: Option<String>,
    pub(crate) validation_only: bool,
    pub(crate) output_file: Option<String>,
}
