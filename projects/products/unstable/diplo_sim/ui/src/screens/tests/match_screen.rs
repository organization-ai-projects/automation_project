use crate::screens::match_screen::screen_subtitle;

#[test]
fn screen_subtitle_is_stable() {
    assert_eq!(screen_subtitle(), "Deterministic diplomacy simulation UI");
}
