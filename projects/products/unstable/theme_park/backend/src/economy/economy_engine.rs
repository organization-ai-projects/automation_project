#![allow(dead_code)]
use crate::economy::budget::Budget;
use crate::sim::sim_state::SimState;
use crate::time::tick::Tick;

/// Reconciles budget from ride and shop revenue each tick.
pub struct EconomyEngine;

impl EconomyEngine {
    pub fn tick(state: &mut SimState, _tick: Tick) {
        // Recompute cumulative revenue from rides and shops into the budget.
        // This is idempotent — budget accumulates delta each tick via ride/shop engines.
        // Here we ensure the budget balance reflects the latest per-tick revenues.
        let ride_revenue: u32 = state.rides.values().map(|r| r.total_revenue).sum();
        let shop_revenue: u32 = state.shops.values().map(|s| s.total_revenue).sum();
        let total = (ride_revenue + shop_revenue) as i64;

        // Budget is maintained cumulatively; we just store snapshot here.
        state.budget.total_revenue = total;
        state.budget.balance = state.budget_initial + total - state.budget.total_expenses;
    }

    pub fn apply_revenue(budget: &mut Budget, amount: u32) {
        budget.add_revenue(amount);
    }
}
