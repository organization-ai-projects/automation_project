pub struct StatusBanner;

impl StatusBanner {
    pub fn print(message: &str) {
        eprintln!("[market_tycoon] {message}");
    }

    pub fn format(message: &str) -> String {
        format!("[market_tycoon] {message}")
    }
}
