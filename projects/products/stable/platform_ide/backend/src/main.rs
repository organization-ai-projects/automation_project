// projects/products/stable/platform_ide/backend/src/main.rs
mod app;
mod auth;
mod changes;
mod client;
mod diff;
mod editor;
mod errors;
mod issues;
mod offline;
mod slices;
mod verification;

#[cfg(test)]
mod tests;

use anyhow::Context;
use app::{IdeApp, IdeConfig};
use auth::Session;
use changes::{ChangeSet, change_set::PatchEntry};
use client::PlatformClient;
use editor::FileBuffer;
use errors::IdeError;
use issues::IssueSummary;
use offline::OfflinePolicy;
use slices::SliceManifest;
use verification::{FindingSeverity, RawFinding, VerificationResultView};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = IdeConfig::from_env();

    tracing::info!(
        platform_url = %config.platform_url,
        "Platform IDE backend starting"
    );

    run_local_bootstrap(&config).context("local bootstrap failed")?;
    run_remote_bootstrap(&config).await;

    tracing::info!("Platform IDE backend initialised");

    Ok(())
}

fn run_local_bootstrap(config: &IdeConfig) -> Result<(), IdeError> {
    let manifest = SliceManifest::new(
        "local-issue",
        "local-base",
        ["src/main.rs", "README.md", "src/lib.rs"],
    );
    let allowed = manifest.allow("src/main.rs")?;

    let mut buffer = FileBuffer::open(allowed, b"fn main() {}\n".to_vec());
    buffer.write(b"fn main() {\n    println!(\"ok\");\n}\n".to_vec());
    let diff = IdeApp::local_diff(&buffer);
    let changed_lines = diff
        .lines
        .iter()
        .filter(|line| {
            matches!(
                line,
                diff::local_diff::DiffLine::Added(_) | diff::local_diff::DiffLine::Removed(_)
            )
        })
        .count();
    tracing::info!(
        path = %diff.path,
        changed_lines,
        has_changes = diff.has_changes(),
        "computed local diff"
    );

    let mut change_set = ChangeSet::new();
    let staged = change_set.add_buffer(&buffer);
    if staged {
        change_set.validate()?;
    }
    tracing::info!(
        staged_entries = change_set.len(),
        has_staged_changes = !change_set.is_empty(),
        "prepared local change set"
    );

    let staged_entries = change_set.entries().to_vec();
    let serialized = common_json::to_string(&staged_entries).unwrap_or_default();
    let parsed: Vec<PatchEntry> = common_json::from_json_str(&serialized).unwrap_or_default();
    tracing::debug!(
        serialized_entries = parsed.len(),
        "validated patch serialization"
    );

    let verification = VerificationResultView::from_raw(
        false,
        [
            RawFinding {
                severity: FindingSeverity::Warning,
                summary: "Local lint warning".to_string(),
                path: Some("src/main.rs".to_string()),
                line: Some(1),
            },
            RawFinding {
                severity: FindingSeverity::Info,
                summary: "Ignored hidden path".to_string(),
                path: Some("secret/internal.rs".to_string()),
                line: Some(3),
            },
        ],
        &manifest,
    );
    tracing::info!(
        healthy = verification.healthy,
        visible_findings = verification.findings.len(),
        "computed verification view"
    );
    let healthy = VerificationResultView::healthy();
    tracing::debug!(healthy = healthy.healthy, "healthy verification baseline");

    let issue = IssueSummary {
        id: "local-issue".to_string(),
        name: "Local bootstrap issue".to_string(),
        description: Some("bootstrap path wiring".to_string()),
        created_at: 0,
    };
    tracing::debug!(issue_id = %issue.id, issue_name = %issue.name, "issue summary wired");

    let policy = OfflinePolicy::disabled();
    let allowed_offline = policy.is_allowed();
    let _offline_gate = policy.require_allowed();

    let session = Session::new("local-token", "local-user");
    let mut app = IdeApp::new(config.platform_url.clone(), session);
    app.active_issue = Some(issue.id.clone());
    app.manifest = Some(manifest);
    app.offline_policy = policy;
    let current_user = app.current_user().to_string();
    let visible_paths = app.slice_manifest().map(|m| m.len()).unwrap_or_default();
    let _ = app.allow_path("src/main.rs")?;
    let _ = buffer.original();
    buffer.revert();
    tracing::info!(
        current_user,
        visible_paths,
        show_offline_controls = app.show_offline_controls(),
        offline_allowed = allowed_offline,
        "local IDE state bootstrapped"
    );
    let _ = IdeError::BufferNotOpen;

    Ok(())
}

async fn run_remote_bootstrap(config: &IdeConfig) {
    let enabled = std::env::var("PLATFORM_IDE_ENABLE_REMOTE_BOOTSTRAP")
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);
    if !enabled {
        tracing::info!(
            "remote bootstrap disabled (set PLATFORM_IDE_ENABLE_REMOTE_BOOTSTRAP=true to run API flow)"
        );
        return;
    }

    let subject = std::env::var("PLATFORM_IDE_SUBJECT").unwrap_or_else(|_| "ide-user".to_string());
    let bootstrap_secret = std::env::var("PLATFORM_IDE_BOOTSTRAP_SECRET").unwrap_or_default();
    if bootstrap_secret.is_empty() {
        tracing::warn!("remote bootstrap enabled but PLATFORM_IDE_BOOTSTRAP_SECRET is empty");
        return;
    }

    let client = PlatformClient::new(config.platform_url.clone());
    let session = match client
        .authenticate(subject.clone(), bootstrap_secret.clone())
        .await
    {
        Ok(session) => session,
        Err(error) => {
            tracing::warn!(%error, "authenticate failed during remote bootstrap");
            return;
        }
    };

    let mut app = IdeApp::new(config.platform_url.clone(), session.clone());
    let issues = match app.list_issues().await {
        Ok(issues) => issues,
        Err(error) => {
            tracing::warn!(%error, "list_issues failed during remote bootstrap");
            return;
        }
    };
    tracing::info!(issues = issues.len(), "remote issue listing completed");

    let Some(first_issue) = issues.first() else {
        tracing::info!("no visible issues for remote bootstrap");
        return;
    };

    if let Err(error) = app.open_issue(first_issue.id.clone()).await {
        tracing::warn!(%error, issue_id = %first_issue.id, "open_issue failed");
        return;
    }

    if let Some(manifest) = app.slice_manifest() {
        let first_path = manifest.iter().next().map(|p| p.to_string());
        let path_count = manifest.len();
        let is_empty = manifest.is_empty();
        tracing::info!(path_count, is_empty, ?first_path, "remote manifest loaded");

        if let Some(path) = first_path
            && let Ok(mut buffer) = app.open_file(&path).await
        {
            let updated = buffer.content().to_vec();
            buffer.write(updated);
            let mut change_set = ChangeSet::new();
            let added = change_set.add_buffer(&buffer);
            tracing::info!(added, staged = change_set.len(), "remote buffer staged");

            if !change_set.is_empty() {
                match app
                    .submit_changes(&change_set, "chore: bootstrap submit check")
                    .await
                {
                    Ok(commit_id) => tracing::info!(%commit_id, "remote submit completed"),
                    Err(error) => tracing::warn!(%error, "submit_changes failed"),
                }
            }
        }

        match app.request_verification().await {
            Ok(result) => tracing::info!(
                healthy = result.healthy,
                findings = result.findings.len(),
                "remote verification completed"
            ),
            Err(error) => tracing::warn!(%error, "request_verification failed"),
        }
    }

    let offline = client.get_offline_policy(&session).await;
    tracing::info!(
        subject = %subject,
        offline_allowed = offline.allowed,
        "remote bootstrap completed"
    );
}
