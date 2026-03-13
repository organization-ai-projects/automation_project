use std::path::{Path, PathBuf};

pub struct WorkspaceScanner;

impl WorkspaceScanner {
    pub fn discover_products(root: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        for segment in ["stable", "unstable"] {
            let base = root.join("projects").join("products").join(segment);
            let read_dir = match std::fs::read_dir(base) {
                Ok(rd) => rd,
                Err(_) => continue,
            };
            for entry in read_dir.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    out.push(path);
                }
            }
        }
        out.sort();
        out
    }

    pub fn discover_tools(root: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        let base = root.join("tools");
        let read_dir = match std::fs::read_dir(base) {
            Ok(rd) => rd,
            Err(_) => return out,
        };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("Cargo.toml").exists() {
                out.push(path);
            }
        }
        out.sort();
        out
    }
}
