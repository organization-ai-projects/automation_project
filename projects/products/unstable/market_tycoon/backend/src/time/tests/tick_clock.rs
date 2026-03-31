use crate::time::tick::Tick;
use crate::time::tick_clock::TickClock;

#[test]
fn clock_starts_at_zero() {
    let clock = TickClock::new(42, 10);
    assert_eq!(clock.current(), Tick(0));
}

#[test]
fn clock_advances() {
    let mut clock = TickClock::new(42, 10);
    clock.advance();
    assert_eq!(clock.current(), Tick(1));
}

#[test]
fn clock_is_done_at_total() {
    let mut clock = TickClock::new(42, 2);
    assert!(!clock.is_done());
    clock.advance();
    assert!(!clock.is_done());
    clock.advance();
    assert!(clock.is_done());
}
