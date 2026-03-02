use crate::diagnostics::error::DocError;
use crate::model::doc_id::DocId;
use crate::model::document::Document;
use crate::replay::doc_event::DocEvent;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocSnapshot {
    pub doc_id: DocId,
    pub version: u64,
    pub document: Document,
    pub events: Vec<DocEvent>,
    pub checksum: String,
}

impl DocSnapshot {
    pub fn create(doc: &Document, version: u64, events: Vec<DocEvent>) -> Result<Self, DocError> {
        let json =
            serde_json::to_string(doc).map_err(|e| DocError::Serialization(e.to_string()))?;
        let checksum = compute_sha256(&json);
        Ok(Self {
            doc_id: doc.id.clone(),
            version,
            document: doc.clone(),
            events,
            checksum,
        })
    }

    pub fn verify(&self) -> Result<(), DocError> {
        let json = serde_json::to_string(&self.document)
            .map_err(|e| DocError::Serialization(e.to_string()))?;
        let expected = compute_sha256(&json);
        if expected == self.checksum {
            Ok(())
        } else {
            Err(DocError::ChecksumMismatch)
        }
    }
}

fn compute_sha256(data: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::doc_id::DocId;
    use crate::model::document::Document;

    #[test]
    fn test_snapshot_determinism() {
        let doc = Document::new(DocId::new("doc1"), "Hello");
        let snap1 = DocSnapshot::create(&doc, 1, vec![]).unwrap();
        let snap2 = DocSnapshot::create(&doc, 1, vec![]).unwrap();
        assert_eq!(snap1.checksum, snap2.checksum);
    }
}
