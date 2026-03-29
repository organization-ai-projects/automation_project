//! tools/versioning_automation/src/issues/tests/auto_link_relation_snapshot.rs
use crate::issues::auto_link_relation_snapshot::AutoLinkRelationSnapshot;

#[test]
fn test_auto_link_relation_snapshot() {
    let payload = "{\"data\":{\"repository\":{\"child\":{\"id\":\"child_id\",\"parent\":{\"number\":123,\"id\":\"parent_id\"}},\"parent\":{\"id\":\"parent_issue_id\"}}}}";
    let snapshot = AutoLinkRelationSnapshot::from_payload(payload);

    assert_eq!(snapshot.current_parent_number(), "123");
    assert_eq!(snapshot.current_parent_node_id(), "parent_id");
    assert_eq!(snapshot.child_node_id(), "child_id");
    assert_eq!(snapshot.parent_node_id(), "parent_issue_id");
}

#[test]
fn test_auto_link_relation_snapshot_empty_payload() {
    let payload = "{}";
    let snapshot = AutoLinkRelationSnapshot::from_payload(payload);

    assert_eq!(snapshot.current_parent_number(), "");
    assert_eq!(snapshot.current_parent_node_id(), "");
    assert_eq!(snapshot.child_node_id(), "");
    assert_eq!(snapshot.parent_node_id(), "");
}
