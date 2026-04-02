//! projects/products/unstable/market_tycoon/ui/src/components/status_banner.rs
pub(crate) struct StatusBanner;

impl StatusBanner {
    pub(crate) fn print(message: &str) {
        eprintln!("[market_tycoon] {message}");
    }

    pub(crate) fn format(message: &str) -> String {
        format!("[market_tycoon] {message}")
    }
}
