use crate::gh_cli;

pub(crate) fn upsert_issue_comment_by_marker(
    repo: &str,
    issue_number: &str,
    marker: &str,
    body: &str,
) -> Result<bool, String> {
    let comments_endpoint = format!("repos/{repo}/issues/{issue_number}/comments");
    let jq_filter = build_marker_jq_filter(marker);
    let comment_id =
        gh_cli::output_trim(&["api", &comments_endpoint, "--paginate", "--jq", &jq_filter])
            .unwrap_or_default();

    let mut args = vec!["api".to_string()];
    if comment_id.trim().is_empty() {
        args.push(comments_endpoint);
    } else {
        args.push("-X".to_string());
        args.push("PATCH".to_string());
        args.push(format!(
            "repos/{repo}/issues/comments/{}",
            comment_id.trim()
        ));
    }
    args.push("-f".to_string());
    args.push(format!("body={body}"));

    let borrowed = args.iter().map(String::as_str).collect::<Vec<_>>();
    gh_cli::status(&borrowed).map(|_| !comment_id.trim().is_empty())
}

pub(crate) fn build_marker_jq_filter(marker: &str) -> String {
    let marker_query = marker.replace('\\', "\\\\").replace('"', "\\\"");
    format!(
        "map(select((.body // \"\") | contains(\"{marker_query}\"))) | sort_by(.updated_at) | last | .id // empty"
    )
}
