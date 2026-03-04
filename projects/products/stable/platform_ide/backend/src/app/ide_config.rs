/// Configuration for the platform IDE.
pub struct IdeConfig {
    /// URL of the platform-versioning backend (e.g. `"http://127.0.0.1:8080"`).
    pub platform_url: String,
}

impl IdeConfig {
    /// Reads configuration from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        let platform_url = std::env::var("PLATFORM_IDE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
        Self { platform_url }
    }
}
