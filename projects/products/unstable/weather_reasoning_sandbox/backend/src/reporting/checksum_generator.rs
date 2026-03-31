use sha2::{Digest, Sha256};

use crate::domain::checksum_value::ChecksumValue;

pub struct ChecksumGenerator;

impl ChecksumGenerator {
    pub fn compute(data: &str) -> ChecksumValue {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        ChecksumValue::new(hex::encode(hasher.finalize()))
    }
}
