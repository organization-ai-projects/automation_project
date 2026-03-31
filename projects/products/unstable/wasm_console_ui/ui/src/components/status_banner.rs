/// Describes a status banner for rendering.
pub struct StatusBanner {
    pub status_message: Option<String>,
    pub error_message: Option<String>,
}

impl StatusBanner {
    pub fn new(status_message: Option<String>, error_message: Option<String>) -> Self {
        Self {
            status_message,
            error_message,
        }
    }

    pub fn has_error(&self) -> bool {
        self.error_message.is_some()
    }

    pub fn has_status(&self) -> bool {
        self.status_message.is_some()
    }

    pub fn display_text(&self) -> &str {
        if let Some(err) = &self.error_message {
            err.as_str()
        } else if let Some(status) = &self.status_message {
            status.as_str()
        } else {
            ""
        }
    }
}
