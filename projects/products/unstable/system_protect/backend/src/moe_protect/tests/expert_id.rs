use crate::moe_protect::expert_id::ExpertId;

#[test]
fn expert_id_display() {
    let id = ExpertId::new("test-expert");
    assert_eq!(format!("{id}"), "test-expert");
}
