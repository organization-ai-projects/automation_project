#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    DocumentLoaded { doc_id: String },
}
