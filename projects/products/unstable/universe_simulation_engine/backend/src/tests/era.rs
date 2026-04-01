use crate::cosmology::era::Era;
use crate::cosmology::era_transition::EraTransition;

#[test]
fn all_eras_exist() {
    let all = Era::all();
    assert_eq!(all.len(), 15);
    assert_eq!(all[0], Era::Singularity);
    assert_eq!(all[14], Era::HeatDeath);
}

#[test]
fn era_transitions() {
    assert_eq!(Era::Singularity.next(), Some(Era::Inflation));
    assert_eq!(Era::HeatDeath.next(), None);
}

#[test]
fn era_for_tick_first_era() {
    let era = EraTransition::era_for_tick(0, 50);
    assert_eq!(era, Era::Singularity);
}

#[test]
fn era_for_tick_second_era() {
    let era = EraTransition::era_for_tick(50, 50);
    assert_eq!(era, Era::Inflation);
}

#[test]
fn era_for_tick_last_era() {
    let era = EraTransition::era_for_tick(1000, 50);
    assert_eq!(era, Era::HeatDeath);
}

#[test]
fn era_progress_start() {
    let progress = EraTransition::era_progress(0, 50);
    assert!((progress - 0.0).abs() < 1e-10);
}

#[test]
fn era_progress_mid() {
    let progress = EraTransition::era_progress(25, 50);
    assert!((progress - 0.5).abs() < 1e-10);
}

#[test]
fn era_display_names() {
    assert_eq!(Era::Singularity.display_name(), "Singularity");
    assert_eq!(Era::HeatDeath.display_name(), "Heat Death");
    assert_eq!(Era::StarFormation.display_name(), "Star Formation");
}

#[test]
fn era_index_roundtrip() {
    for era in Era::all() {
        let idx = era.index();
        assert_eq!(Era::all()[idx], *era);
    }
}
