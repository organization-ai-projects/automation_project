#![allow(dead_code)]
use crate::events::event_log::EventLog;
use crate::events::sim_event::SimEvent;
use crate::shops::shop_id::ShopId;
use crate::sim::sim_state::SimState;
use crate::time::tick::Tick;
use crate::visitors::visitor::VisitorStatus;
use crate::visitors::visitor_id::VisitorId;

/// Processes all shops for one tick (deterministic — shops sorted by ShopId).
pub struct ShopEngine;

impl ShopEngine {
    pub fn tick(state: &mut SimState, event_log: &mut EventLog, tick: Tick) {
        let shop_ids: Vec<ShopId> = {
            let mut ids: Vec<ShopId> = state.shops.keys().copied().collect();
            ids.sort();
            ids
        };

        for sid in shop_ids {
            Self::tick_shop(state, event_log, tick, sid);
        }
    }

    fn tick_shop(
        state: &mut SimState,
        event_log: &mut EventLog,
        tick: Tick,
        sid: ShopId,
    ) {
        // Find visitors currently shopping here; advance their timer.
        let shopping_visitors: Vec<VisitorId> = state
            .visitors
            .values()
            .filter(|v| matches!(&v.status, VisitorStatus::Shopping { shop, .. } if *shop == sid))
            .map(|v| v.id)
            .collect();

        for vid in shopping_visitors {
            let done = {
                let v = state.visitors.get_mut(&vid).unwrap();
                if let VisitorStatus::Shopping {
                    ref mut ticks_remaining,
                    ..
                } = v.status
                {
                    if *ticks_remaining > 0 {
                        *ticks_remaining -= 1;
                    }
                    *ticks_remaining == 0
                } else {
                    false
                }
            };

            if done {
                let price = state.shops[&sid].price;
                {
                    let v = state.visitors.get_mut(&vid).unwrap();
                    v.status = VisitorStatus::Idle;
                    v.mood = v.mood.adjust(5);
                    v.revenue_generated += price;
                }
                {
                    let shop = state.shops.get_mut(&sid).unwrap();
                    shop.total_revenue += price;
                    shop.total_customers += 1;
                }
                event_log.push(SimEvent::VisitorShopVisit {
                    tick,
                    visitor_id: vid,
                    shop_id: sid,
                    spent: price,
                });
            }
        }
    }
}
