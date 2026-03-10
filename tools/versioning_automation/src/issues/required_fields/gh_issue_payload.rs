use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GhIssuePayload {
    pub(crate) labels: Option<Vec<GhIssueLabel>>,
    pub(crate) title: Option<String>,
    pub(crate) body: Option<String>,
}

impl GhIssuePayload {
    pub(crate) fn join_labels(&self) -> String {
        self.labels
            .as_ref()
            .map(|labels| {
                labels
                    .iter()
                    .filter_map(|label| label.name.clone())
                    .collect::<Vec<_>>()
                    .join("||")
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GhIssueLabel {
    pub(crate) name: Option<String>,
}
