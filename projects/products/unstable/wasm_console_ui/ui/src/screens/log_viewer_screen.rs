/// Describes what the log viewer screen needs for rendering.
pub struct LogViewerScreen {
    pub title: String,
    pub content: Option<String>,
}

impl LogViewerScreen {
    pub fn new(content: Option<String>) -> Self {
        Self {
            title: "Log Viewer".to_string(),
            content,
        }
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    pub fn line_count(&self) -> usize {
        self.content
            .as_ref()
            .map(|c| c.lines().count())
            .unwrap_or(0)
    }
}
