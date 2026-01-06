use std::fmt;

#[derive(Debug)]
pub struct AutopilotError {
    pub message: String,
}

impl fmt::Display for AutopilotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl From<String> for AutopilotError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<AutopilotError> for String {
    fn from(error: AutopilotError) -> Self {
        format!("AutopilotError: {}", error).to_string()
    }
}
