#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    LoadDocument { doc_id: String },
}
