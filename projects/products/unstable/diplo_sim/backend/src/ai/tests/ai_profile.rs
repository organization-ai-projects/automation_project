use crate::ai::ai_profile::AiProfile;

#[test]
fn ai_profile_default_move_probability_is_50() {
    let profile = AiProfile::default();
    assert_eq!(profile.move_probability, 50);
}
