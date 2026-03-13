use std::process::Command;

use serde::Deserialize;

use crate::issues::commands::SyncProjectStatusOptions;

pub(crate) fn run_sync_project_status(opts: SyncProjectStatusOptions) -> i32 {
    #[derive(Debug, Deserialize)]
    struct IssueProjectItemsResponse {
        data: Option<IssueProjectItemsData>,
    }
    #[derive(Debug, Deserialize)]
    struct IssueProjectItemsData {
        repository: Option<IssueProjectItemsRepository>,
    }
    #[derive(Debug, Deserialize)]
    struct IssueProjectItemsRepository {
        issue: Option<IssueProjectItemsIssue>,
    }
    #[derive(Debug, Deserialize)]
    struct IssueProjectItemsIssue {
        #[serde(rename = "projectItems")]
        project_items: Option<IssueProjectItemsConnection>,
    }
    #[derive(Debug, Deserialize)]
    struct IssueProjectItemsConnection {
        nodes: Vec<ProjectItemNode>,
    }
    #[derive(Debug, Deserialize)]
    struct ProjectItemNode {
        id: Option<String>,
        project: Option<ProjectNode>,
    }
    #[derive(Debug, Deserialize)]
    struct ProjectNode {
        id: Option<String>,
        title: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    struct ProjectFieldsResponse {
        data: Option<ProjectFieldsData>,
    }
    #[derive(Debug, Deserialize)]
    struct ProjectFieldsData {
        node: Option<ProjectFieldsNode>,
    }
    #[derive(Debug, Deserialize)]
    struct ProjectFieldsNode {
        fields: Option<ProjectFieldsConnection>,
    }
    #[derive(Debug, Deserialize)]
    struct ProjectFieldsConnection {
        nodes: Vec<ProjectStatusFieldNode>,
    }
    #[derive(Debug, Deserialize)]
    struct ProjectStatusFieldNode {
        id: Option<String>,
        name: Option<String>,
        options: Option<Vec<ProjectStatusOption>>,
    }
    #[derive(Debug, Deserialize)]
    struct ProjectStatusOption {
        id: Option<String>,
        name: Option<String>,
    }

    let Some((owner, name)) = split_repo(&opts.repo) else {
        return 0;
    };

    let Some(issue_payload) = fetch_issue_project_items(owner, name, &opts.issue) else {
        return 0;
    };
    let Ok(issue_response) =
        common_json::from_json_str::<IssueProjectItemsResponse>(&issue_payload)
    else {
        return 0;
    };

    let project_items = issue_response
        .data
        .and_then(|data| data.repository)
        .and_then(|repository| repository.issue)
        .and_then(|issue| issue.project_items)
        .map(|connection| connection.nodes)
        .unwrap_or_default();

    for item in project_items {
        let item_id = item.id.unwrap_or_default();
        let project = item.project;
        let project_id = project
            .as_ref()
            .and_then(|value| value.id.clone())
            .unwrap_or_default();
        let project_title = project.and_then(|value| value.title).unwrap_or_default();

        if item_id.is_empty() || project_id.is_empty() {
            continue;
        }

        let Some(project_payload) = fetch_project_fields(&project_id) else {
            continue;
        };
        let Ok(project_response) =
            common_json::from_json_str::<ProjectFieldsResponse>(&project_payload)
        else {
            continue;
        };

        let mut status_field_id = String::new();
        let mut status_option_id = String::new();
        let fields = project_response
            .data
            .and_then(|data| data.node)
            .and_then(|node| node.fields)
            .map(|fields| fields.nodes)
            .unwrap_or_default();
        for field in fields {
            let field_name = field.name.unwrap_or_default();
            if field_name != "Status" {
                continue;
            }
            status_field_id = field.id.unwrap_or_default();
            let options = field.options.unwrap_or_default();
            for option in options {
                let option_name = option.name.unwrap_or_default();
                if option_name.eq_ignore_ascii_case(&opts.status) {
                    status_option_id = option.id.unwrap_or_default();
                    break;
                }
            }
            break;
        }

        if status_field_id.is_empty() || status_option_id.is_empty() {
            continue;
        }

        if update_project_status(&project_id, &item_id, &status_field_id, &status_option_id) {
            println!(
                "Issue #{}: synced project '{}' status to '{}'.",
                opts.issue, project_title, opts.status
            );
        }
    }

    0
}

fn split_repo(repo: &str) -> Option<(&str, &str)> {
    let mut parts = repo.splitn(2, '/');
    let owner = parts.next().unwrap_or_default();
    let name = parts.next().unwrap_or_default();
    if owner.is_empty() || name.is_empty() {
        None
    } else {
        Some((owner, name))
    }
}

fn fetch_issue_project_items(owner: &str, name: &str, issue_number: &str) -> Option<String> {
    let query = r#"
    query($owner:String!, $name:String!, $number:Int!) {
      repository(owner:$owner, name:$name) {
        issue(number:$number) {
          projectItems(first:50) {
            nodes {
              id
              project { id title }
            }
          }
        }
      }
    }"#;
    gh_graphql(&[
        ("-f", &format!("query={query}")),
        ("-f", &format!("owner={owner}")),
        ("-f", &format!("name={name}")),
        ("-F", &format!("number={issue_number}")),
    ])
}

fn fetch_project_fields(project_id: &str) -> Option<String> {
    let query = r#"
      query($projectId:ID!) {
        node(id:$projectId) {
          ... on ProjectV2 {
            fields(first:100) {
              nodes {
                ... on ProjectV2SingleSelectField {
                  id
                  name
                  options { id name }
                }
              }
            }
          }
        }
      }"#;
    gh_graphql(&[
        ("-f", &format!("query={query}")),
        ("-f", &format!("projectId={project_id}")),
    ])
}

fn update_project_status(project_id: &str, item_id: &str, field_id: &str, option_id: &str) -> bool {
    let query = r#"
      mutation($project:ID!, $item:ID!, $field:ID!, $option: String!) {
        updateProjectV2ItemFieldValue(input: {
          projectId: $project
          itemId: $item
          fieldId: $field
          value: { singleSelectOptionId: $option }
        }) { projectV2Item { id } }
      }"#;
    gh_graphql(&[
        ("-f", &format!("query={query}")),
        ("-f", &format!("project={project_id}")),
        ("-f", &format!("item={item_id}")),
        ("-f", &format!("field={field_id}")),
        ("-f", &format!("option={option_id}")),
    ])
    .is_some()
}

fn gh_graphql(args: &[(&str, &str)]) -> Option<String> {
    let mut cmd = Command::new("gh");
    cmd.arg("api").arg("graphql");
    for (flag, value) in args {
        cmd.arg(flag);
        cmd.arg(value);
    }
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}
