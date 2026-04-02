use crate::events::event_log::EventLog;
use crate::events::sim_event::SimEvent;
use crate::model::company::Company;
use crate::model::good::Good;
use crate::model::inventory::Inventory;
use crate::pricing::price::Price;
use crate::pricing::pricing_policy::PricingPolicy;
use crate::time::tick::Tick;

pub struct PricingEngine;

impl PricingEngine {
    pub fn update_prices(
        tick: &Tick,
        _company: &Company,
        inventory: &Inventory,
        policy: &PricingPolicy,
        event_log: &mut EventLog,
    ) {
        for (good, &qty) in inventory.stock() {
            let base_cost = Self::base_cost(good);
            let markup = base_cost * policy.markup_percent as i64 / 100;
            let mut final_price = base_cost + markup;

            if qty > policy.discount_threshold {
                let discount = final_price * policy.discount_percent as i64 / 100;
                final_price -= discount;
            }

            event_log.push(SimEvent::price_updated(
                *tick,
                inventory.store_id(),
                *good,
                Price::new(final_price),
            ));
        }
    }

    fn base_cost(good: &Good) -> i64 {
        match good {
            Good::Widget => 500,
            Good::Gadget => 1200,
            Good::Gizmo => 800,
        }
    }
}
