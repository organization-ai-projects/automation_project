#[derive(Debug)]
pub(crate) struct SyncMainDevCiOptions {
    pub(crate) remote: String,
    pub(crate) main: String,
    pub(crate) dev: String,
    pub(crate) sync_branch: String,
}
