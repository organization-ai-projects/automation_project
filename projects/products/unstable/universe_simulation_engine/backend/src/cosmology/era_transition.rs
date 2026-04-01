use crate::cosmology::era::Era;

pub struct EraTransition;

impl EraTransition {
    pub fn era_for_tick(tick: u64, ticks_per_era: u64) -> Era {
        if ticks_per_era == 0 {
            return Era::Singularity;
        }
        let era_index = (tick / ticks_per_era) as usize;
        let all = Era::all();
        if era_index >= all.len() {
            all[all.len() - 1]
        } else {
            all[era_index]
        }
    }

    pub fn era_progress(tick: u64, ticks_per_era: u64) -> f64 {
        if ticks_per_era == 0 {
            return 0.0;
        }
        (tick % ticks_per_era) as f64 / ticks_per_era as f64
    }
}
