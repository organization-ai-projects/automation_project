use crate::issue_remote_snapshot::{issue_labels_raw, load_issue_remote_snapshot};
use crate::issues::non_compliance_reason_from_content;
use crate::pr::commands::pr_issue_context_options::PrIssueContextOptions;
use crate::pr::resolve_category::issue_category_from_title;

pub(crate) fn run_issue_context(opts: PrIssueContextOptions) -> i32 {
    let payload = load_issue_context_payload(&opts);
    println!("{}\x1f{}\x1f{}", payload.0, payload.1, payload.2);
    0
}

pub(crate) fn load_issue_context_payload(opts: &PrIssueContextOptions) -> (String, String, String) {
    let Ok(snapshot) = load_issue_remote_snapshot(&opts.issue_number, opts.repo.as_deref()) else {
        return (String::new(), "Unknown".to_string(), String::new());
    };
    let labels_raw = issue_labels_raw(&snapshot);

    let title_category = if snapshot.title.trim().is_empty() {
        "Unknown".to_string()
    } else {
        issue_category_from_title(&snapshot.title).to_string()
    };

    let reason = compute_non_compliance_reason(&snapshot.title, &snapshot.body, &labels_raw);

    (labels_raw, title_category, reason)
}

fn compute_non_compliance_reason(title: &str, body: &str, labels_raw: &str) -> String {
    non_compliance_reason_from_content(title, body, labels_raw).unwrap_or_default()
}
