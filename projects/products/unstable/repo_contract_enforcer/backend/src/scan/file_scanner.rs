#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileScanner;

impl FileScanner {
    pub fn gather_rs_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
        let mut out = Vec::new();
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let read_dir = match std::fs::read_dir(dir) {
                Ok(rd) => rd,
                Err(_) => continue,
            };
            for entry in read_dir.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    out.push(path);
                }
            }
        }
        out.sort();
        out
    }

    pub fn gather_named_entries(root: &std::path::Path, name: &str) -> Vec<std::path::PathBuf> {
        let mut out = Vec::new();
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let read_dir = match std::fs::read_dir(dir) {
                Ok(rd) => rd,
                Err(_) => continue,
            };
            for entry in read_dir.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if path.file_name().and_then(|n| n.to_str()) == Some(name) {
                        out.push(path.clone());
                    }
                    stack.push(path);
                } else if path.file_stem().and_then(|n| n.to_str()) == Some(name)
                    && path.extension().and_then(|e| e.to_str()) == Some("rs")
                {
                    out.push(path);
                }
            }
        }
        out.sort();
        out
    }
}
