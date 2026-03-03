use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RefView {
    pub full_name: String,
    pub short_name: String,
    pub kind: crate::refs_store::RefKind,
    pub commit_id: String,
    pub stable_ref_id: Option<String>,
}
