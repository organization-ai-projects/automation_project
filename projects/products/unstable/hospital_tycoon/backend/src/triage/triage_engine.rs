// projects/products/unstable/hospital_tycoon/backend/src/triage/triage_engine.rs
use crate::model::patient_id::PatientId;
use crate::model::room_id::RoomId;
use crate::patients::patient::Patient;
use crate::rooms::room::Room;
use crate::rooms::room_kind::RoomKind;
use crate::rooms::room_queue::RoomQueue;
use crate::triage::triage_rule::TriageRule;
use std::collections::BTreeMap;

pub struct TriageEngine {
    pub rules: Vec<TriageRule>,
}

impl TriageEngine {
    pub fn new(rules: Vec<TriageRule>) -> Self {
        Self { rules }
    }

    pub fn default_rules() -> Self {
        Self { rules: vec![] }
    }

    /// Route a waiting patient to the best available room.
    /// Deterministic tie-break: lowest RoomId when multiple rooms have equal queue length.
    pub fn route(
        &self,
        patient_id: PatientId,
        patient: &Patient,
        rooms: &BTreeMap<RoomId, Room>,
        queues: &mut BTreeMap<RoomId, RoomQueue>,
    ) -> Option<RoomId> {
        let target_kind = self.target_kind_for(patient);

        // Find rooms of target kind with capacity available, sorted by queue length then RoomId
        let mut candidates: Vec<(usize, RoomId)> = rooms
            .iter()
            .filter(|(_, r)| r.kind == target_kind)
            .filter_map(|(rid, room)| {
                let q_len = queues.get(rid).map(|q| q.len()).unwrap_or(0);
                if (q_len as u32) < room.capacity {
                    Some((q_len, *rid))
                } else {
                    None
                }
            })
            .collect();

        // Sort by queue length asc, then by RoomId asc — deterministic tie-break
        candidates.sort_by_key(|&(q_len, rid)| (q_len, rid));

        if let Some((_, room_id)) = candidates.first() {
            let room_id = *room_id;
            queues
                .entry(room_id)
                .or_insert_with(|| RoomQueue::new(room_id))
                .enqueue(patient_id);
            Some(room_id)
        } else {
            None
        }
    }

    fn target_kind_for(&self, patient: &Patient) -> RoomKind {
        for rule in &self.rules {
            if rule.disease_id == patient.disease.id {
                return rule.target_room_kind.clone();
            }
        }
        // Default: route to Treatment
        RoomKind::Treatment
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patients::disease::Disease;
    use crate::patients::disease_id::DiseaseId;

    fn make_patient(id: u32) -> (PatientId, Patient) {
        let pid = PatientId::new(id);
        let patient = Patient {
            id: pid,
            name: format!("P{}", id),
            disease: Disease {
                id: DiseaseId::new("flu"),
                name: "Influenza".to_string(),
                severity: 2,
            },
            severity: 2,
            assigned_room: None,
            tick_admitted: 0,
        };
        (pid, patient)
    }

    fn make_treatment_room(id: u32, capacity: u32) -> (RoomId, Room) {
        let rid = RoomId::new(id);
        let room = Room {
            id: rid,
            kind: RoomKind::Treatment,
            capacity,
            staff_slots: 1,
        };
        (rid, room)
    }

    #[test]
    fn triage_routes_deterministically() {
        let engine = TriageEngine::default_rules();
        let mut rooms: BTreeMap<RoomId, Room> = BTreeMap::new();
        let mut queues: BTreeMap<RoomId, RoomQueue> = BTreeMap::new();

        let (rid1, r1) = make_treatment_room(1, 5);
        let (rid2, r2) = make_treatment_room(2, 5);
        rooms.insert(rid1, r1);
        rooms.insert(rid2, r2);
        queues.insert(rid1, RoomQueue::new(rid1));
        queues.insert(rid2, RoomQueue::new(rid2));

        let (pid1, p1) = make_patient(1);
        let (pid2, p2) = make_patient(2);
        let (pid3, p3) = make_patient(3);

        // First two patients should go to room 1 and room 2 alternately (fill evenly, tie-break by id)
        let r = engine.route(pid1, &p1, &rooms, &mut queues);
        assert_eq!(r, Some(rid1)); // room 1 has lower id, picked first when both empty

        let r = engine.route(pid2, &p2, &rooms, &mut queues);
        assert_eq!(r, Some(rid2)); // room 2 now has fewer patients

        let r = engine.route(pid3, &p3, &rooms, &mut queues);
        assert_eq!(r, Some(rid1)); // rooms equal, pick lowest id

        // Verify determinism: same inputs produce same result
        let mut queues2: BTreeMap<RoomId, RoomQueue> = BTreeMap::new();
        queues2.insert(rid1, RoomQueue::new(rid1));
        queues2.insert(rid2, RoomQueue::new(rid2));
        let r2 = engine.route(pid1, &p1, &rooms, &mut queues2);
        assert_eq!(r2, Some(rid1));
    }
}
