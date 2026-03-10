#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ValidationEntry {
    pub(crate) code: String,
    pub(crate) field: String,
    pub(crate) message: String,
}

impl ValidationEntry {
    pub(crate) fn new(code: &str, field: String, message: String) -> Self {
        Self {
            code: code.to_string(),
            field,
            message,
        }
    }

    pub(crate) fn as_pipe_line(&self) -> String {
        format!("{}|{}|{}", self.code, self.field, self.message)
    }
}
