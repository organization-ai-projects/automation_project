/// CI context information
#[derive(Debug, Clone)]
pub struct CiContext {
    pub available: bool,
    pub info: String,
}

/// CI adapter (read-only in V0)
#[derive(Debug)]
pub struct CiAdapter;

impl CiAdapter {
    /// Create a new CI adapter
    pub fn new() -> Self {
        Self
    }

    /// Get CI context (read-only)
    pub fn get_context(&self) -> Result<CiContext, String> {
        // V0: Stub implementation
        Ok(CiContext {
            available: false,
            info: "CI adapter not implemented in V0 (read-only stub)".to_string(),
        })
    }
}

impl Default for CiAdapter {
    fn default() -> Self {
        Self::new()
    }
}
