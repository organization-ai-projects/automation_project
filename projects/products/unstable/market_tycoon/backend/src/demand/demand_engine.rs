use std::collections::BTreeMap;

use crate::demand::customer_segment::CustomerSegment;
use crate::demand::demand_model::DemandModel;
use crate::events::event_log::EventLog;
use crate::events::sim_event::SimEvent;
use crate::finance::ledger::Ledger;
use crate::finance::transaction::Transaction;
use crate::model::inventory::Inventory;
use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

pub struct DemandEngine;

impl DemandEngine {
    pub fn process_demand(
        tick: &Tick,
        model: &DemandModel,
        segments: &[CustomerSegment],
        rng_val: u64,
        inventories: &mut BTreeMap<StoreId, Inventory>,
        ledger: &mut Ledger,
        event_log: &mut EventLog,
    ) {
        for segment in segments {
            let demand = Self::compute_demand(segment, model, rng_val);
            for (sid, inv) in inventories.iter_mut() {
                let price = inv.get_price(&segment.good).map(|p| p.cents()).unwrap_or(model.base_price_reference);
                let units_to_buy = demand.min(inv.get_stock(&segment.good));
                if units_to_buy > 0 && inv.remove_stock(segment.good, units_to_buy) {
                    let revenue = price * units_to_buy as i64;
                    ledger.record(Transaction::sale(*tick, *sid, revenue));
                    event_log.push(SimEvent::sale(*tick, *sid, segment.good, units_to_buy, revenue));
                }
            }
        }
    }

    fn compute_demand(segment: &CustomerSegment, model: &DemandModel, rng_val: u64) -> u64 {
        let noise = (rng_val % 21) as i64 - 10;
        let base = segment.base_demand as i64;
        let adjusted = base + noise * base / 100;
        adjusted.max(0) as u64
    }
}
