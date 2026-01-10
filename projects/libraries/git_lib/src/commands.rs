// projects/libraries/git_lib/src/commands.rs
use command_runner::{CommandError, run_cmd_allow_failure, run_cmd_ok};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

use crate::git_change::{GitChange, GitStatus};

type Result<T> = StdResult<T, CommandError>;

const GIT_BIN: &str = "git";
const ADD_CHUNK: usize = 100;

// -----------------------------
// Small helpers
// -----------------------------

#[inline]
fn s_trim(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

/// UTF-8 safe truncation (no panic).
pub fn truncate_utf8(mut s: String, max: usize) -> String {
    if s.len() <= max {
        return s;
    }
    let mut cut = 0usize;
    for (i, _) in s.char_indices() {
        if i > max {
            break;
        }
        cut = i;
    }
    s.truncate(cut);
    s
}

fn invalid(reason: impl Into<String>) -> CommandError {
    CommandError::InvalidInput {
        program: GIT_BIN.to_string(),
        reason: reason.into(),
    }
}

// -----------------------------
// Repo checks / info
// -----------------------------

/// Ensures the given path is a Git repository.
pub fn ensure_git_repo(repo_path: &Path, logs: &mut Vec<String>) -> Result<()> {
    let out = run_cmd_ok(
        repo_path,
        GIT_BIN,
        &["rev-parse", "--is-inside-work-tree"],
        logs,
    )?;
    let s = s_trim(&out.stdout);
    if s != "true" {
        return Err(invalid("Refus: pas dans un repository git."));
    }
    Ok(())
}

/// Default repo path if empty. (Does not canonicalize on purpose.)
pub fn normalize_repo_path(repo_path: &Path) -> Result<PathBuf> {
    if repo_path.as_os_str().is_empty() {
        Ok(PathBuf::from("."))
    } else {
        Ok(repo_path.to_path_buf())
    }
}

/// Retrieves the current branch and checks if HEAD is detached.
pub fn current_branch(repo_path: &Path, logs: &mut Vec<String>) -> Result<(String, bool)> {
    let out = run_cmd_ok(
        repo_path,
        GIT_BIN,
        &["rev-parse", "--abbrev-ref", "HEAD"],
        logs,
    )?;
    let branch = s_trim(&out.stdout);
    let detached = branch == "HEAD" || branch.is_empty();
    Ok((branch, detached))
}

// -----------------------------
// Index / commit / push
// -----------------------------

/// Adds files to the Git index.
pub fn git_add_paths(repo_path: &Path, paths: &[String], logs: &mut Vec<String>) -> Result<()> {
    logs.push("[debug] git_add_paths: function entered".to_string());

    let cleaned: Vec<&str> = paths
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    if cleaned.is_empty() {
        return Ok(());
    }

    let relative_paths: Vec<String> = cleaned
        .iter()
        .filter(|p| repo_path.join(p).exists())
        .map(|p| {
            repo_path
                .join(p)
                .strip_prefix(repo_path)
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect();

    for chunk in relative_paths.chunks(ADD_CHUNK) {
        let mut args = Vec::<&str>::with_capacity(2 + chunk.len());
        args.push("add");
        args.push("--");
        args.extend(chunk.iter().map(String::as_str));

        run_cmd_ok(repo_path, GIT_BIN, &args, logs)?;
    }

    Ok(())
}

/// Commits staged changes with a subject and optional body.
/// Note: this does NOT stage files. Caller should stage first.
pub fn git_commit(
    repo_path: &Path,
    subject: &str,
    body: &str,
    logs: &mut Vec<String>,
) -> Result<()> {
    let subject = subject.trim();
    if subject.is_empty() {
        return Err(invalid("Sujet de commit vide"));
    }

    // Safety check: make sure something is staged (gives a clearer error)
    let staged = run_cmd_ok(
        repo_path,
        GIT_BIN,
        &["diff", "--cached", "--name-only"],
        logs,
    )?;
    if staged.stdout.is_empty() {
        return Err(invalid("Aucun fichier mis en scène pour le commit"));
    }

    if body.trim().is_empty() {
        run_cmd_ok(repo_path, GIT_BIN, &["commit", "-m", subject], logs)?;
    } else {
        run_cmd_ok(
            repo_path,
            GIT_BIN,
            &["commit", "-m", subject, "-m", body],
            logs,
        )?;
    }

    Ok(())
}

/// Validates commit without applying changes.
/// This requires staged files too (git checks it).
pub fn git_commit_dry_run(
    repo_path: &Path,
    subject: &str,
    body: &str,
    logs: &mut Vec<String>,
) -> Result<()> {
    let subject = subject.trim();
    if subject.is_empty() {
        return Err(invalid("Sujet de commit vide (dry-run)"));
    }

    if body.trim().is_empty() {
        run_cmd_ok(
            repo_path,
            GIT_BIN,
            &["commit", "--dry-run", "-m", subject],
            logs,
        )?;
    } else {
        run_cmd_ok(
            repo_path,
            GIT_BIN,
            &["commit", "--dry-run", "-m", subject, "-m", body],
            logs,
        )?;
    }

    Ok(())
}

/// Pushes the current branch.
/// If upstream missing:
/// - if set_upstream_if_missing => push -u <remote> <branch>
/// - else => no-op (logged)
pub fn git_push_current_branch(
    repo_path: &Path,
    remote: &str,
    set_upstream_if_missing: bool,
    logs: &mut Vec<String>,
) -> Result<()> {
    let remote = remote.trim();
    if remote.is_empty() {
        return Err(invalid("Remote vide pour le push"));
    }

    // Upstream probe is intentionally permissive:
    // non-zero means "no upstream", not necessarily a fatal error.
    let upstream_probe = run_cmd_allow_failure(
        repo_path,
        GIT_BIN,
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
        logs,
    )?;

    let has_upstream = upstream_probe.status.success();

    if has_upstream {
        run_cmd_ok(repo_path, GIT_BIN, &["push"], logs)?;
        return Ok(());
    }

    if !set_upstream_if_missing {
        logs.push("[git] upstream missing; push skipped (set_upstream_if_missing=false)".into());
        return Ok(());
    }

    let (branch, detached) = current_branch(repo_path, logs)?;
    if detached {
        return Err(invalid(
            "Refus: HEAD détaché, impossible de push upstream automatiquement.",
        ));
    }

    run_cmd_ok(repo_path, GIT_BIN, &["push", "-u", remote, &branch], logs)?;
    Ok(())
}

/// Pushes to a remote branch.
pub fn git_push(
    repo_path: &Path,
    remote: &str,
    branch: &str,
    logs: &mut Vec<String>,
) -> Result<()> {
    let remote = remote.trim();
    let branch = branch.trim();

    if remote.is_empty() {
        return Err(invalid("Remote vide pour le push"));
    }
    if branch.is_empty() {
        return Err(invalid("Branch vide pour le push"));
    }

    run_cmd_ok(repo_path, GIT_BIN, &["push", remote, branch], logs)?;
    Ok(())
}

// -----------------------------
// Status / parsing
// -----------------------------

/// Retrieves the Git status in porcelain v1 format (-z) and parses it.
pub fn git_status_porcelain_z(repo_path: &Path, logs: &mut Vec<String>) -> Result<Vec<GitChange>> {
    let out = run_cmd_ok(repo_path, GIT_BIN, &["status", "--porcelain", "-z"], logs)?;
    logs.push(format!(
        "[debug] git status --porcelain -z: {} bytes",
        out.stdout.len()
    ));
    logs.push(format!(
        "[debug] git_status_porcelain_z: raw output={:?}",
        String::from_utf8_lossy(&out.stdout)
    ));

    let parsed_changes = parse_porcelain_v1_z(&out.stdout)?;
    logs.push(format!(
        "[debug] git_status_porcelain_z: parsed changes={:?}",
        parsed_changes
    ));

    Ok(parsed_changes)
}

/// Parses `git status --porcelain -z` (porcelain v1).
/// Strict parser: any format deviation returns Err.
pub fn parse_porcelain_v1_z(bytes: &[u8]) -> Result<Vec<GitChange>> {
    let mut i = 0usize;
    let mut out = Vec::new();

    while i < bytes.len() {
        if i + 3 > bytes.len() {
            return Err(invalid(
                "Parse error: truncated entry header in porcelain -z output",
            ));
        }

        let x = bytes[i];
        let y = bytes[i + 1];
        let space = bytes[i + 2];
        i += 3;

        if space != b' ' {
            return Err(invalid(
                "Parse error: expected space after XY in porcelain -z output",
            ));
        }

        let (field1, next_i) = read_cstr(bytes, i)?;
        i = next_i;

        let xy = [x, y];

        // Rename/copy is encoded in X (index status) in porcelain v1.
        let is_rename_like = matches!(x, b'R' | b'C');

        if is_rename_like {
            let orig = field1;
            let (field2, next_i2) = read_cstr(bytes, i)?;
            i = next_i2;

            out.push(GitChange {
                xy,
                path: field2,
                orig_path: Some(orig),
            });
        } else {
            out.push(GitChange {
                xy,
                path: field1,
                orig_path: None,
            });
        }
    }

    Ok(out)
}

/// Reads a NUL-terminated string from bytes starting at `start`.
pub fn read_cstr(bytes: &[u8], start: usize) -> Result<(String, usize)> {
    let mut end = start;
    while end < bytes.len() && bytes[end] != 0 {
        end += 1;
    }
    if end >= bytes.len() {
        return Err(invalid(
            "Parse error: expected NUL terminator in porcelain -z output",
        ));
    }
    let s = String::from_utf8_lossy(&bytes[start..end]).to_string();
    Ok((s, end + 1))
}

// -----------------------------
// Commit message builders
// -----------------------------

/// Builds a short commit subject.
pub fn build_commit_subject(branch: &str) -> String {
    let base = format!("varina: update ({})", branch.trim());
    truncate_utf8(base, 72)
}

/// Builds a detailed commit message.
/// Grouping by GitStatus avoids allocations for status keys.
pub fn build_commit_message(branch: &str, relevant: &[GitChange]) -> (String, String) {
    let subject = build_commit_subject(branch);

    let mut by_status: BTreeMap<GitStatus, Vec<String>> = BTreeMap::new();
    for c in relevant {
        by_status
            .entry(c.status())
            .or_default()
            .push(display_change_path(c));
    }

    let mut body_lines = Vec::new();
    for (status, files) in by_status {
        body_lines.push(format!("- {}:", status.as_str()));
        for f in files.into_iter().take(50) {
            body_lines.push(format!("  - {}", f));
        }
        if body_lines.len() > 400 {
            body_lines.push("  - ... (truncated)".into());
            break;
        }
    }

    let body = truncate_utf8(body_lines.join("\n"), 4000);
    (subject, body)
}

/// Displays a Git change path (rename friendly).
pub fn display_change_path(ch: &GitChange) -> String {
    match &ch.orig_path {
        Some(orig) => format!("{orig} -> {}", ch.path),
        None => ch.path.clone(),
    }
}
