use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn source_has_no_shell_validation_regressions() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs_files(&root, &mut files);

    let forbidden = ["/bin/sh", "--validation-command", "--validation-shell"];
    let mut violations = Vec::new();

    for file in files {
        let content = fs::read_to_string(&file).expect("read source file");
        for token in forbidden {
            if content.contains(token) {
                violations.push(format!("{} contains '{}'", file.display(), token));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Shell regression detected:\n{}",
        violations.join("\n")
    );
}

fn collect_rs_files(root: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(root).expect("read_dir");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}
