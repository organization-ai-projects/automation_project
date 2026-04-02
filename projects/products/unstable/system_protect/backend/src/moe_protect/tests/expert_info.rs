use crate::moe_protect::expert_id::ExpertId;
use crate::moe_protect::expert_info::ExpertInfo;
use crate::moe_protect::expert_type::ExpertType;

#[test]
fn expert_info_holds_fields() {
    let info = ExpertInfo {
        id: ExpertId::new("av"),
        name: "Antivirus".to_string(),
        expert_type: ExpertType::Antivirus,
    };
    assert_eq!(info.name, "Antivirus");
}
