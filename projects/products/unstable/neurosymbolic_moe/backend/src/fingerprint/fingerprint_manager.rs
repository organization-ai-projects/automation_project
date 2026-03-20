use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct FingerprintManager {
    fingerprints: VecDeque<String>,
    capacity: usize,
}

impl FingerprintManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            fingerprints: VecDeque::new(),
            capacity: capacity.max(1),
        }
    }

    pub fn add_fingerprint(&mut self, fingerprint: String) {
        self.fingerprints
            .retain(|existing| existing != &fingerprint);
        self.fingerprints.push_back(fingerprint);
        while self.fingerprints.len() > self.capacity {
            self.fingerprints.pop_front();
        }
    }

    pub fn contains(&self, fingerprint: &str) -> bool {
        self.fingerprints.iter().any(|stored| stored == fingerprint)
    }

    pub fn count(&self) -> usize {
        self.fingerprints.len()
    }
}
