use crate::replay::event_log::EventLog;
use crate::replay::replay_event::ReplayEvent;
use crate::time::turn::Turn;

#[test]
fn event_log_push_appends_event() {
    let mut event_log = EventLog::new();
    event_log.push(ReplayEvent {
        turn: Turn::new(0),
        order_sets: vec![],
    });

    assert_eq!(event_log.events.len(), 1);
    assert_eq!(event_log.events[0].turn, Turn::new(0));
}
