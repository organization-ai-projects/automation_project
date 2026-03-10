use std::process::Command;

pub(crate) fn resolve_repo_name(explicit_repo: Option<String>) -> Result<String, String> {
    resolve_repo_name_optional(explicit_repo.as_deref())
        .ok_or_else(|| "Error: unable to determine repository.".to_string())
}

pub(crate) fn resolve_repo_name_optional(explicit_repo: Option<&str>) -> Option<String> {
    if let Some(repo) = explicit_repo.and_then(non_empty) {
        return Some(repo.to_string());
    }

    if let Ok(env_repo) = std::env::var("GH_REPO")
        && let Some(repo) = non_empty(&env_repo)
    {
        return Some(repo.to_string());
    }

    let output = Command::new("gh")
        .arg("repo")
        .arg("view")
        .arg("--json")
        .arg("nameWithOwner")
        .arg("-q")
        .arg(".nameWithOwner")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let repo_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    non_empty(&repo_name).map(str::to_string)
}

fn non_empty(value: &str) -> Option<&str> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}
