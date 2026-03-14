//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/pipeline_moe/trainer_trigger_queue_state.rs
use crate::moe_core::MoeError;
use crate::orchestrator::TrainerTriggerEvent;
use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
pub(in crate::orchestrator) struct TrainerTriggerQueueState {
    events: VecDeque<TrainerTriggerEvent>,
    max_events: usize,
    leased_event_ids: HashSet<u64>,
}

impl TrainerTriggerQueueState {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: VecDeque::new(),
            max_events: max_events.max(1),
            leased_event_ids: HashSet::new(),
        }
    }

    pub fn with_events(max_events: usize, events: Vec<TrainerTriggerEvent>) -> Self {
        let mut queue = Self::new(max_events);
        for event in events {
            queue.push(event);
        }
        queue
    }

    pub fn max_events(&self) -> usize {
        self.max_events
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn events(&self) -> &VecDeque<TrainerTriggerEvent> {
        &self.events
    }

    pub fn leased_count(&self) -> usize {
        self.leased_event_ids.len()
    }

    pub fn max_delivery_attempts(&self) -> u32 {
        self.events
            .iter()
            .map(|event| event.delivery_attempts)
            .max()
            .unwrap_or(0)
    }

    pub fn oldest_generated_at(&self) -> Option<u64> {
        self.events.iter().map(|event| event.generated_at).min()
    }

    pub fn newest_generated_at(&self) -> Option<u64> {
        self.events.iter().map(|event| event.generated_at).max()
    }

    pub fn pop_next(&mut self) -> Option<TrainerTriggerEvent> {
        let event = self.events.pop_front()?;
        self.leased_event_ids.remove(&event.event_id);
        Some(event)
    }

    pub fn lease_next(
        &mut self,
        now_epoch_seconds: u64,
        min_retry_delay_seconds: u64,
    ) -> Option<TrainerTriggerEvent> {
        let mut leased_idx = None;
        for (idx, event) in self.events.iter().enumerate() {
            if self.leased_event_ids.contains(&event.event_id) {
                continue;
            }
            let eligible = event.last_attempted_at.is_none_or(|last| {
                now_epoch_seconds >= last.saturating_add(min_retry_delay_seconds)
            });
            if eligible {
                leased_idx = Some(idx);
                break;
            }
        }
        let idx = leased_idx?;
        let event = self.events.get_mut(idx)?;
        event.delivery_attempts = event.delivery_attempts.saturating_add(1);
        event.last_attempted_at = Some(now_epoch_seconds);
        self.leased_event_ids.insert(event.event_id);
        Some(event.clone())
    }

    pub fn acknowledge(&mut self, event_id: u64) -> bool {
        if !self.leased_event_ids.contains(&event_id) {
            return false;
        }
        if let Some(idx) = self
            .events
            .iter()
            .position(|event| event.event_id == event_id)
        {
            self.events.remove(idx);
            self.leased_event_ids.remove(&event_id);
            true
        } else {
            false
        }
    }

    pub fn mark_delivery_failed(&mut self, event_id: u64, failed_at_epoch_seconds: u64) -> bool {
        if !self.leased_event_ids.contains(&event_id) {
            return false;
        }
        if let Some(event) = self
            .events
            .iter_mut()
            .find(|event| event.event_id == event_id)
        {
            event.last_attempted_at = Some(failed_at_epoch_seconds);
            self.leased_event_ids.remove(&event_id);
            true
        } else {
            false
        }
    }

    pub fn drain(&mut self, max_events: usize) -> Vec<TrainerTriggerEvent> {
        if max_events == 0 || self.events.is_empty() {
            return Vec::new();
        }
        let drain_len = max_events.min(self.events.len());
        let mut drained = Vec::with_capacity(drain_len);
        for _ in 0..drain_len {
            if let Some(event) = self.events.pop_front() {
                self.leased_event_ids.remove(&event.event_id);
                drained.push(event);
            }
        }
        drained
    }

    pub fn push(&mut self, event: TrainerTriggerEvent) {
        self.events.push_back(event);
        while self.events.len() > self.max_events {
            if let Some(removed) = self.events.pop_front() {
                self.leased_event_ids.remove(&removed.event_id);
            }
        }
    }

    pub fn validate_invariants(&self) -> Result<(), MoeError> {
        if self.events.len() > self.max_events {
            return Err(MoeError::PolicyRejected(format!(
                "trainer trigger queue invariant failed: pending events exceed max ({} > {})",
                self.events.len(),
                self.max_events
            )));
        }

        let mut ids = HashSet::new();
        for event in &self.events {
            if !ids.insert(event.event_id) {
                return Err(MoeError::PolicyRejected(format!(
                    "trainer trigger queue invariant failed: duplicate event_id {}",
                    event.event_id
                )));
            }
        }

        if self
            .leased_event_ids
            .iter()
            .any(|leased_id| !ids.contains(leased_id))
        {
            return Err(MoeError::PolicyRejected(
                "trainer trigger queue invariant failed: leased event id missing from queue"
                    .to_string(),
            ));
        }

        Ok(())
    }
}
