/// GitHub context information
#[derive(Debug, Clone)]
pub struct GhContext {
    pub available: bool,
    pub info: String,
}

/// GitHub adapter (read-only in V0)
#[derive(Debug)]
pub struct GhAdapter;

impl GhAdapter {
    /// Create a new GitHub adapter
    pub fn new() -> Self {
        Self
    }

    /// Get GitHub context (read-only)
    pub fn get_context(&self) -> Result<GhContext, String> {
        // V0: Just check if gh CLI is available
        // In a real implementation, we'd use `gh --json` commands
        Ok(GhContext {
            available: false,
            info: "GitHub adapter not implemented in V0 (read-only stub)".to_string(),
        })
    }
}

impl Default for GhAdapter {
    fn default() -> Self {
        Self::new()
    }
}
