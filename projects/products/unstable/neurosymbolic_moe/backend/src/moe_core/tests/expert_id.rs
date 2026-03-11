use crate::moe_core::ExpertId;

#[test]
fn expert_id_new_and_as_str() {
    let id = ExpertId::new("expert-1");
    assert_eq!(id.as_str(), "expert-1");
}
