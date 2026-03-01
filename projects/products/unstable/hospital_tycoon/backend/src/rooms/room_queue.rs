// projects/products/unstable/hospital_tycoon/backend/src/rooms/room_queue.rs
use crate::model::patient_id::PatientId;
use crate::model::room_id::RoomId;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomQueue {
    pub room_id: RoomId,
    pub queue: VecDeque<PatientId>,
}

impl RoomQueue {
    pub fn new(room_id: RoomId) -> Self {
        Self {
            room_id,
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, patient_id: PatientId) {
        self.queue.push_back(patient_id);
    }

    /// Dequeues the next patient. Deterministic: FIFO order.
    pub fn dequeue(&mut self) -> Option<PatientId> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_fifo_determinism() {
        let room_id = RoomId::new(1);
        let mut q = RoomQueue::new(room_id);
        q.enqueue(PatientId::new(3));
        q.enqueue(PatientId::new(1));
        q.enqueue(PatientId::new(2));
        assert_eq!(q.dequeue(), Some(PatientId::new(3)));
        assert_eq!(q.dequeue(), Some(PatientId::new(1)));
        assert_eq!(q.dequeue(), Some(PatientId::new(2)));
    }
}
