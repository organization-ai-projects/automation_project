#[derive(Debug)]
pub(crate) struct CiWatchPrOptions {
    pub(crate) pr_number: Option<String>,
    pub(crate) poll_interval: u64,
    pub(crate) max_wait: u64,
}
