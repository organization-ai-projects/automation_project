//! tools/versioning_automation/src/issues/auto_link_relation_snapshot.rs
use serde::Deserialize;

pub(crate) struct AutoLinkRelationSnapshot(String, String, String, String);

impl AutoLinkRelationSnapshot {
    pub(crate) fn from_payload(payload: &str) -> Self {
        #[derive(Debug, Deserialize)]
        struct GraphqlRelationPayload {
            data: Option<GraphqlRelationData>,
        }
        #[derive(Debug, Deserialize)]
        struct GraphqlRelationData {
            repository: Option<GraphqlRelationRepository>,
        }
        #[derive(Debug, Deserialize)]
        struct GraphqlRelationRepository {
            child: Option<GraphqlRelationChild>,
            parent: Option<GraphqlRelationParentIssue>,
        }
        #[derive(Debug, Deserialize)]
        struct GraphqlRelationChild {
            id: Option<String>,
            parent: Option<GraphqlRelationParentRef>,
        }
        #[derive(Debug, Deserialize)]
        struct GraphqlRelationParentRef {
            number: Option<u64>,
            id: Option<String>,
        }
        #[derive(Debug, Deserialize)]
        struct GraphqlRelationParentIssue {
            id: Option<String>,
        }

        let Ok(json) = common_json::from_json_str::<GraphqlRelationPayload>(payload) else {
            return Self(String::new(), String::new(), String::new(), String::new());
        };
        let repository = json.data.and_then(|data| data.repository);
        let child = repository.as_ref().and_then(|repo| repo.child.as_ref());
        let parent_ref = child.and_then(|child| child.parent.as_ref());
        let parent_issue = repository.as_ref().and_then(|repo| repo.parent.as_ref());

        Self(
            parent_ref
                .and_then(|parent| parent.number)
                .map(|value| value.to_string())
                .unwrap_or_default(),
            parent_ref
                .and_then(|parent| parent.id.clone())
                .unwrap_or_default(),
            child.and_then(|entry| entry.id.clone()).unwrap_or_default(),
            parent_issue
                .and_then(|parent| parent.id.clone())
                .unwrap_or_default(),
        )
    }

    pub(crate) fn current_parent_number(&self) -> &str {
        &self.0
    }

    pub(crate) fn current_parent_node_id(&self) -> &str {
        &self.1
    }

    pub(crate) fn child_node_id(&self) -> &str {
        &self.2
    }

    pub(crate) fn parent_node_id(&self) -> &str {
        &self.3
    }
}
