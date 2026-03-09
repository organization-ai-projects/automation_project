use crate::actions::action::Action;
use crate::model::candidate_id::CandidateId;
use crate::replay::action_entry::ActionEntry;

#[test]
fn action_entry_keeps_payload() {
    let entry = ActionEntry {
        day: 3,
        candidate_id: CandidateId::new("a"),
        action: Action::MediaAppearance,
    };

    assert_eq!(entry.day, 3);
    assert_eq!(entry.candidate_id.to_string(), "a");
}
