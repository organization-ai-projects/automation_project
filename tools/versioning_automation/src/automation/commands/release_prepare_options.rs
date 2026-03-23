#[derive(Debug, Clone)]
pub(crate) struct ReleasePrepareOptions {
    pub(crate) version: String,
    pub(crate) auto_changelog: bool,
}
