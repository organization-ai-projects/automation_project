use crate::data::type_chart::TypeChart;
use crate::data::type_id::TypeId;
use crate::rng::rng_draw::RngDraw;
use rand::Rng;
use rand::rngs::SmallRng;

pub struct DamageCalc;

impl DamageCalc {
    pub fn calculate(
        rng: &mut SmallRng,
        power: u32,
        attack: u32,
        defense: u32,
        level: u32,
        move_type: &TypeId,
        defender_types: &[TypeId],
        type_chart: &TypeChart,
        step: u64,
        label: &str,
        draws: &mut Vec<RngDraw>,
    ) -> u32 {
        if power == 0 || defense == 0 {
            return 0;
        }
        let base = ((2 * level / 5 + 2) * power * attack) / (defense * 50) + 2;
        let effectiveness = type_chart.compute_effectiveness(move_type, defender_types);
        let eff_damage = (base as f64 * effectiveness) as u32;

        let roll: u64 = rng.random_range(85..101);
        draws.push(RngDraw::new(step, label, roll, 101));
        let final_damage = eff_damage * roll as u32 / 100;
        final_damage.max(1)
    }
}
