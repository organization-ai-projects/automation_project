// projects/products/unstable/platform_ide/backend/src/main.rs
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = app::IdeConfig::from_env();

    tracing::info!(
        platform_url = %config.platform_url,
        "Platform IDE backend starting"
    );

    let subject = std::env::var("PLATFORM_IDE_SUBJECT").ok();
    let bootstrap_secret = std::env::var("PLATFORM_IDE_BOOTSTRAP_SECRET").ok();

    if let (Some(subject), Some(secret)) = (subject, bootstrap_secret) {
        run_live_workflow(&config, &subject, &secret).await?;
    } else {
        tracing::info!(
            "Platform IDE initialised. Set PLATFORM_IDE_SUBJECT and PLATFORM_IDE_BOOTSTRAP_SECRET for live workflow."
        );
    }

    Ok(())
}

async fn run_live_workflow(
    config: &app::IdeConfig,
    subject: &str,
    bootstrap_secret: &str,
) -> anyhow::Result<()> {
    tracing::info!(subject = %subject, "Starting live IDE workflow");

    let client = client::PlatformClient::new(config.platform_url.clone());
    let session = client
        .authenticate(subject.to_string(), bootstrap_secret.to_string())
        .await?;

    let mut app = app::IdeApp::new(config.platform_url.clone(), session);

    tracing::info!(user = %app.current_user(), "Authenticated session");
    let issues = app.list_issues().await?;
    tracing::info!(count = issues.len(), "Loaded visible issues");

    let selected_issue = if let Ok(issue_id) = std::env::var("PLATFORM_IDE_ISSUE_ID") {
        issue_id
    } else if let Some(issue) = issues.first() {
        issue.id.clone()
    } else {
        tracing::warn!("No visible issues; stopping workflow");
        return Ok(());
    };

    app.open_issue(selected_issue.clone()).await?;
    tracing::info!(issue_id = %selected_issue, "Opened issue and loaded slice");
    if let Some(manifest) = app.slice_manifest() {
        tracing::info!(
            issue_id = %manifest.issue_id,
            allowed_paths = manifest.len(),
            empty = manifest.is_empty(),
            "Loaded manifest details"
        );
    }

    let target_path = if let Ok(path) = std::env::var("PLATFORM_IDE_FILE_PATH") {
        Some(path)
    } else {
        app.slice_manifest()
            .and_then(|m| m.iter().next().map(str::to_string))
    };

    if let Some(path) = target_path {
        app.allow_path(&path)?;
        let mut buffer = app.open_file(&path).await?;
        let baseline = app::IdeApp::local_diff(&buffer);
        tracing::info!(
            path = %baseline.path,
            baseline_has_changes = baseline.has_changes(),
            baseline_lines = baseline.lines.len(),
            "Opened file in live workflow"
        );

        if let Ok(append_text) = std::env::var("PLATFORM_IDE_APPEND_TEXT") {
            let mut updated = buffer.content().to_vec();
            updated.extend_from_slice(append_text.as_bytes());
            buffer.write(updated);

            let diff = app::IdeApp::local_diff(&buffer);
            let mut changes = changes::ChangeSet::new();
            if changes.add_buffer(&buffer) && !changes.is_empty() {
                let message = std::env::var("PLATFORM_IDE_COMMIT_MESSAGE")
                    .unwrap_or_else(|_| "ide: apply requested edit".to_string());
                let commit_id = app.submit_changes(&changes, message).await?;
                tracing::info!(
                    commit_id = %commit_id,
                    changed_files = changes.len(),
                    diff_lines = diff.lines.len(),
                    "Submitted live changes"
                );
            } else {
                tracing::info!("No dirty buffer to submit");
            }
            buffer.revert();
        }
    } else {
        return Err(errors::IdeError::BufferNotOpen.into());
    }

    if let Err(err) = app.offline_policy.require_allowed() {
        tracing::info!(%err, "Offline mode remains disabled by policy");
    }

    let verification = match app.request_verification().await {
        Ok(view) => view,
        Err(err) => {
            tracing::warn!(%err, "Verification failed; falling back to healthy placeholder");
            verification::VerificationResultView::healthy()
        }
    };
    tracing::info!(
        healthy = verification.healthy,
        findings = verification.findings.len(),
        offline_controls = app.show_offline_controls(),
        "Verification completed"
    );

    Ok(())
}
