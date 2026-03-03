use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommitView {
    pub kind: crate::objects::ObjectKind,
    pub commit: crate::objects::Commit,
    pub id_raw_len: usize,
}
