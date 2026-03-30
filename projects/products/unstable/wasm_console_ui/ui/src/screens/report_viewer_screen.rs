/// Describes what the report viewer screen needs for rendering.
pub struct ReportViewerScreen {
    pub title: String,
    pub content: Option<String>,
}

impl ReportViewerScreen {
    pub fn new(content: Option<String>) -> Self {
        Self {
            title: "Report Viewer".to_string(),
            content,
        }
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }
}
