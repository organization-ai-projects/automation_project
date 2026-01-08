use std::path::Path;

pub fn is_git_repo(repo_path: &Path) -> bool {
    repo_path.join(".git").exists()
}
