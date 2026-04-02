use std::collections::BTreeMap;

use crate::events::event_log::EventLog;
use crate::logistics::logistics_engine::LogisticsEngine;
use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

#[test]
fn process_shipments_no_crash() {
    let mut inventories = BTreeMap::new();
    let mut log = EventLog::new();
    LogisticsEngine::process_shipments(&Tick(0), &[], &mut inventories, &mut log);
    assert_eq!(log.events().len(), 0);
}
