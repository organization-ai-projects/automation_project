use crate::issues::issue_sync_plan::{plan_done_in_dev_sync, plan_reopen_sync};

#[test]
fn plan_reopen_sync_reopens_closed_issue_and_removes_done_in_dev_when_present() {
    let plan = plan_reopen_sync("CLOSED", true);
    assert!(plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(plan.remove_done_in_dev_label);
}

#[test]
fn plan_reopen_sync_only_removes_done_in_dev_for_open_issue() {
    let plan = plan_reopen_sync("OPEN", true);
    assert!(!plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(plan.remove_done_in_dev_label);
}

#[test]
fn plan_reopen_sync_is_noop_for_open_issue_without_done_in_dev() {
    let plan = plan_reopen_sync("OPEN", false);
    assert!(!plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(!plan.remove_done_in_dev_label);
}

#[test]
fn plan_done_in_dev_sync_adds_done_in_dev_only_for_open_issue_without_label() {
    let plan = plan_done_in_dev_sync("OPEN", false);
    assert!(!plan.reopen_issue);
    assert!(plan.add_done_in_dev_label);
    assert!(!plan.remove_done_in_dev_label);
}

#[test]
fn plan_done_in_dev_sync_skips_done_in_dev_when_label_is_already_present() {
    let plan = plan_done_in_dev_sync("OPEN", true);
    assert!(!plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(!plan.remove_done_in_dev_label);
}

#[test]
fn plan_done_in_dev_sync_skips_done_in_dev_for_closed_issue() {
    let plan = plan_done_in_dev_sync("CLOSED", false);
    assert!(!plan.reopen_issue);
    assert!(!plan.add_done_in_dev_label);
    assert!(!plan.remove_done_in_dev_label);
}
