use crate::events::event_log::EventLog;
use crate::events::sim_event::SimEvent;
use crate::model::good::Good;
use crate::model::store_id::StoreId;
use crate::time::tick::Tick;

#[test]
fn push_and_len() {
    let mut log = EventLog::new();
    assert!(log.is_empty());
    log.push(SimEvent::delivery(Tick(0), StoreId(0), Good::Widget, 10));
    assert_eq!(log.len(), 1);
    assert!(!log.is_empty());
}
