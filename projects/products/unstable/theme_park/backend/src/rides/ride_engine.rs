#![allow(dead_code)]
use crate::events::event_log::EventLog;
use crate::events::sim_event::SimEvent;
use crate::rides::ride::Ride;
use crate::rides::ride_id::RideId;
use crate::sim::sim_state::SimState;
use crate::time::tick::Tick;

/// Processes all rides for one tick (deterministic — rides sorted by RideId).
pub struct RideEngine;

impl RideEngine {
    pub fn tick(state: &mut SimState, event_log: &mut EventLog, tick: Tick) {
        let ride_ids: Vec<RideId> = {
            let mut ids: Vec<RideId> = state.rides.keys().copied().collect();
            ids.sort();
            ids
        };

        for rid in ride_ids {
            Self::tick_ride(state, event_log, tick, rid);
        }
    }

    fn tick_ride(
        state: &mut SimState,
        event_log: &mut EventLog,
        tick: Tick,
        rid: RideId,
    ) {
        // Advance maintenance timer first.
        let maintenance_done = {
            let ride = state.rides.get_mut(&rid).unwrap();
            ride.maintenance.advance_tick()
        };
        if maintenance_done {
            event_log.push(SimEvent::MaintenanceComplete {
                tick,
                ride_id: rid,
            });
        }

        // If under maintenance, do nothing else.
        if !state.rides[&rid].maintenance.is_operational() {
            return;
        }

        // Finish current run if timer elapsed.
        let run_just_finished = {
            let ride = state.rides.get_mut(&rid).unwrap();
            if ride.running {
                if ride.ticks_remaining > 0 {
                    ride.ticks_remaining -= 1;
                }
                ride.ticks_remaining == 0
            } else {
                false
            }
        };

        if run_just_finished {
            // Collect riders, update visitor states.
            let riders: Vec<_> = {
                let ride = state.rides.get_mut(&rid).unwrap();
                ride.running = false;
                ride.current_riders.drain(..).collect()
            };
            for vid in &riders {
                if let Some(v) = state.visitors.get_mut(vid) {
                    v.mood = v.mood.adjust(10);
                    v.patience = crate::visitors::patience::Patience::new(
                        crate::visitors::patience::Patience::INITIAL,
                    );
                    v.rides_completed += 1;
                    v.status = crate::visitors::visitor::VisitorStatus::Idle;
                }
                event_log.push(SimEvent::VisitorExitedRide {
                    tick,
                    visitor_id: *vid,
                    ride_id: rid,
                });
            }
            event_log.push(SimEvent::RideComplete {
                tick,
                ride_id: rid,
                riders_count: riders.len() as u32,
            });
        }

        // Start a new run if queue is non-empty and ride is idle.
        {
            let (not_running, queue_nonempty, cap, ticks) = {
                let ride = state.rides.get(&rid).unwrap();
                (!ride.running, !ride.queue.is_empty(), ride.capacity as usize, ride.ticks_per_ride)
            };
            if not_running && queue_nonempty {
                let ride = state.rides.get_mut(&rid).unwrap();
                let batch = ride.queue.dequeue_batch(cap);
                ride.current_riders = batch.clone();
                ride.ticks_remaining = ticks;
                ride.running = true;
                ride.total_riders_served += batch.len() as u32;
                let price = ride.price;
                ride.total_revenue += batch.len() as u32 * price;

                // Update visitor states to Riding.
                for vid in &batch {
                    if let Some(v) = state.visitors.get_mut(vid) {
                        v.status = crate::visitors::visitor::VisitorStatus::Riding(rid);
                        v.revenue_generated += price;
                    }
                }
                event_log.push(SimEvent::RideStarted {
                    tick,
                    ride_id: rid,
                    riders_count: batch.len() as u32,
                });
            }
        }

        // Deterministic maintenance check: trigger if
        // (tick.value() + ride_id.value()) % maintenance_interval == 0
        // and ride is idle and not already under maintenance.
        {
            let interval = state.config.maintenance_check_interval;
            let trigger = (tick.value() + rid.value() as u64).is_multiple_of(interval) && tick.value() > 0;
            if trigger {
                let ride = state.rides.get_mut(&rid).unwrap();
                if ride.maintenance.is_operational() && !ride.running {
                    ride.maintenance.begin(3);
                    event_log.push(SimEvent::MaintenanceStarted {
                        tick,
                        ride_id: rid,
                    });
                }
            }
        }
    }

    /// Update economy stats after all rides have been processed this tick.
    pub fn update_ride_revenue(ride: &Ride, budget: &mut crate::economy::budget::Budget) {
        let _ = ride; // revenue already accumulated on ride struct; budget synced in economy_engine
        let _ = budget;
    }
}
