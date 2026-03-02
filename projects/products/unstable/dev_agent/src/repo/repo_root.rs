use std::path::PathBuf;

pub struct RepoRoot {
    pub path: PathBuf,
}

impl RepoRoot {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    #[allow(dead_code)]
    pub fn exists(&self) -> bool {
        self.path.is_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stores_path() {
        let r = RepoRoot::new(PathBuf::from("/tmp"));
        assert_eq!(r.path, PathBuf::from("/tmp"));
    }

    #[test]
    fn exists_for_real_dir() {
        let r = RepoRoot::new(PathBuf::from("/tmp"));
        assert!(r.exists());
    }

    #[test]
    fn not_exists_for_missing_dir() {
        let r = RepoRoot::new(PathBuf::from("/nonexistent_path_xyz"));
        assert!(!r.exists());
    }
}
