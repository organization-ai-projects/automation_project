#![allow(dead_code)]
use crate::reputation::reputation::Reputation;
use crate::sim::sim_state::SimState;
use crate::time::tick::Tick;

/// Recomputes reputation from average visitor mood each tick.
pub struct ReputationEngine;

impl ReputationEngine {
    pub fn tick(state: &mut SimState, _tick: Tick) {
        let active: Vec<i32> = state
            .visitors
            .values()
            .filter(|v| v.is_active())
            .map(|v| v.mood.value())
            .collect();

        if active.is_empty() {
            return;
        }

        let avg = active.iter().sum::<i32>() / active.len() as i32;
        // Nudge reputation toward avg mood (smooth by 10%).
        let delta = (avg - state.reputation.score) / 10;
        state.reputation.set(state.reputation.score + delta);
    }

    pub fn compute_final(state: &SimState) -> i32 {
        let all: Vec<i32> = state.visitors.values().map(|v| v.mood.value()).collect();
        if all.is_empty() {
            return Reputation::INITIAL;
        }
        all.iter().sum::<i32>() / all.len() as i32
    }
}
