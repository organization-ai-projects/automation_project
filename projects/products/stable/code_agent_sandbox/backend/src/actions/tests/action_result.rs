use crate::actions::ActionResult;

#[test]
fn action_result_builders_set_expected_flags() {
    let ok = ActionResult::success("ReadFile", "done", None);
    assert!(ok.ok);
    assert_eq!(ok.kind, "ReadFile");

    let err = ActionResult::error("PolicyViolation", "blocked");
    assert!(!err.ok);
    assert_eq!(err.kind, "PolicyViolation");
    assert!(err.data.is_none());
}
