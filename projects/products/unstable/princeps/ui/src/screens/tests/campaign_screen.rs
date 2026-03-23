use crate::screens::campaign_screen::CampaignScreen;

#[test]
fn campaign_screen_summary_line_contains_seed_and_days() {
    let screen = CampaignScreen::new(7, 12, "ready".to_string());
    let summary = screen.summary_line();
    assert!(summary.contains("seed=7"));
    assert!(summary.contains("days=12"));
}
