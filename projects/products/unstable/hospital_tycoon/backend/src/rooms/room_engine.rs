// projects/products/unstable/hospital_tycoon/backend/src/rooms/room_engine.rs
use crate::model::patient_id::PatientId;
use crate::model::room_id::RoomId;
use crate::rooms::room_queue::RoomQueue;
use std::collections::BTreeMap;

pub struct RoomEngine;

impl RoomEngine {
    /// Process one patient from each room queue. Returns list of treated PatientIds per room.
    pub fn process_queues(queues: &mut BTreeMap<RoomId, RoomQueue>) -> Vec<PatientId> {
        let mut treated = Vec::new();
        // BTreeMap iteration is sorted by key (RoomId) — deterministic
        for queue in queues.values_mut() {
            if let Some(pid) = queue.dequeue() {
                treated.push(pid);
            }
        }
        treated
    }
}
