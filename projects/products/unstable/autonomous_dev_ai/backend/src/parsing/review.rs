//! projects/products/unstable/autonomous_dev_ai/src/parsing/review.rs
use crate::{
    error::{AgentError, AgentResult},
    pr_flow::ReviewComment,
};

pub(crate) fn parse_review_comments_from_gh_json(raw: &str) -> AgentResult<Vec<ReviewComment>> {
    let root = common_json::from_str(raw).map_err(|e| {
        AgentError::State(format!("Failed to parse gh review payload as JSON: {}", e))
    })?;

    let mut comments = Vec::new();

    if let Some(reviews) = root.get("reviews").and_then(|v| v.as_array()) {
        for review in reviews {
            let state = review
                .get("state")
                .and_then(|v| v.as_str())
                .unwrap_or("UNKNOWN");
            let reviewer = review
                .get("author")
                .and_then(|v| v.get("login"))
                .and_then(|v| v.as_str())
                .unwrap_or("reviewer")
                .to_string();
            let body = review
                .get("body")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let resolved = !state.eq_ignore_ascii_case("CHANGES_REQUESTED");

            if body.is_empty() {
                if state.eq_ignore_ascii_case("CHANGES_REQUESTED") {
                    comments.push(ReviewComment {
                        reviewer,
                        body: "Changes requested (no details provided)".to_string(),
                        resolved,
                    });
                }
                continue;
            }

            comments.push(ReviewComment {
                reviewer,
                body,
                resolved,
            });
        }
    }

    if let Some(pr_comments) = root.get("comments").and_then(|v| v.as_array()) {
        for comment in pr_comments {
            let body = comment
                .get("body")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            if body.is_empty() {
                continue;
            }
            let reviewer = comment
                .get("author")
                .and_then(|v| v.get("login"))
                .and_then(|v| v.as_str())
                .unwrap_or("commenter")
                .to_string();
            comments.push(ReviewComment {
                reviewer,
                body,
                // Plain PR comments are informational by default.
                resolved: true,
            });
        }
    }

    Ok(comments)
}
