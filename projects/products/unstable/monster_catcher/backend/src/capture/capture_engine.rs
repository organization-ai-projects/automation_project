use crate::capture::capture_roll::CaptureRoll;
use crate::model::monster::Monster;
use crate::rng::rng_draw::RngDraw;
use rand::Rng;
use rand::rngs::SmallRng;

pub struct CaptureEngine;

impl CaptureEngine {
    pub fn attempt_capture(
        rng: &mut SmallRng,
        target: &Monster,
        capture_rate: u32,
        step: u64,
        draws: &mut Vec<RngDraw>,
    ) -> CaptureRoll {
        let hp_factor = if target.max_hp > 0 {
            (target.max_hp - target.current_hp) as u64 * 100 / target.max_hp as u64
        } else {
            100
        };
        let threshold = (capture_rate as u64 * (50 + hp_factor)) / 100;
        let threshold = threshold.min(255);

        let roll: u64 = rng.random_range(0..256);
        draws.push(RngDraw::new(step, "capture_roll", roll, 256));

        let success = roll < threshold;
        CaptureRoll {
            step,
            capture_rate,
            current_hp: target.current_hp,
            max_hp: target.max_hp,
            roll,
            threshold,
            success,
        }
    }
}
