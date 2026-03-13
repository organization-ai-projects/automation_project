#[derive(Debug)]
pub(crate) struct AddCommitPushOptions {
    pub(crate) message: String,
    pub(crate) no_verify: bool,
    pub(crate) remote: String,
}
