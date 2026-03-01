// projects/products/unstable/hospital_tycoon/backend/src/sim/sim_engine.rs
use crate::config::sim_config::SimConfig;
use crate::economy::economy_engine::EconomyEngine;
use crate::economy::pricing::Pricing;
use crate::model::hospital_state::HospitalState;
use crate::model::patient_id::PatientId;
use crate::model::room_id::RoomId;
use crate::model::staff_id::StaffId;
use crate::patients::disease::Disease;
use crate::patients::disease_id::DiseaseId;
use crate::patients::patient::Patient;
use crate::patients::patient_state::PatientState;
use crate::reputation::reputation::Reputation;
use crate::reputation::reputation_engine::ReputationEngine;
use crate::rooms::room::Room;
use crate::rooms::room_engine::RoomEngine;
use crate::rooms::room_kind::RoomKind;
use crate::rooms::room_queue::RoomQueue;
use crate::economy::budget::Budget;
use crate::sim::event_log::EventLog;
use crate::sim::sim_event::SimEvent;
use crate::staff::staff::Staff;
use crate::staff::staff_skill::StaffSkill;
use crate::time::tick_clock::TickClock;
use crate::triage::triage_engine::TriageEngine;
use rand::RngCore;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::collections::BTreeMap;

pub struct SimEngine {
    pub state: HospitalState,
    pub event_log: EventLog,
    pub clock: TickClock,
    rng: StdRng,
    config: SimConfig,
    triage: TriageEngine,
    economy: EconomyEngine,
}

impl SimEngine {
    pub fn new(seed: u64, ticks: u64, config: SimConfig) -> Self {
        let mut rooms: BTreeMap<RoomId, Room> = BTreeMap::new();
        let mut room_queues: BTreeMap<RoomId, RoomQueue> = BTreeMap::new();
        for rc in &config.rooms {
            let rid = RoomId::new(rc.id);
            rooms.insert(rid, Room { id: rid, kind: rc.kind.clone(), capacity: rc.capacity, staff_slots: rc.staff_slots });
            room_queues.insert(rid, RoomQueue::new(rid));
        }

        let mut staff: BTreeMap<StaffId, Staff> = BTreeMap::new();
        for sc in &config.staff {
            let sid = StaffId::new(sc.id);
            staff.insert(sid, Staff {
                id: sid,
                name: sc.name.clone(),
                role: sc.role.clone(),
                skill: StaffSkill { level: sc.skill_level },
            });
        }

        let state = HospitalState {
            patients: BTreeMap::new(),
            treated_patients: Vec::new(),
            rooms,
            room_queues,
            staff,
            budget: Budget::new(config.initial_budget),
            reputation: Reputation::new(config.initial_reputation),
            tick: crate::time::tick::Tick::zero(),
            next_patient_id: 1,
            waiting_patients: Vec::new(),
        };

        Self {
            state,
            event_log: EventLog::new(),
            clock: TickClock::new(seed, ticks),
            rng: StdRng::seed_from_u64(seed),
            triage: TriageEngine::default_rules(),
            economy: EconomyEngine::new(Pricing::default()),
            config,
        }
    }

    /// Advance the simulation by one tick.
    pub fn step_one(&mut self) {
        if self.clock.is_done() {
            return;
        }
        self.clock.tick();
        let tick = self.clock.current_tick();
        self.state.tick = tick;

        // 1. Spawn patients
        self.spawn_patients(tick);

        // 2. Triage: assign waiting patients to rooms
        self.run_triage(tick);

        // 3. Room engine: process one patient from each room queue
        self.run_rooms(tick);
    }

    fn spawn_patients(&mut self, tick: crate::time::tick::Tick) {
        let rate = self.config.patient_spawn_rate;
        if rate > 0 && tick.value() % rate == 0 {
            if self.config.diseases.is_empty() {
                return;
            }
            let idx = (self.rng.next_u64() % self.config.diseases.len() as u64) as usize;
            let dc = &self.config.diseases[idx];
            let pid = PatientId::new(self.state.next_patient_id);
            self.state.next_patient_id += 1;
            let patient = Patient {
                id: pid,
                name: format!("Patient{}", pid.0),
                disease: Disease {
                    id: DiseaseId::new(dc.id.clone()),
                    name: dc.name.clone(),
                    severity: dc.severity,
                },
                severity: dc.severity,
                assigned_room: None,
                tick_admitted: tick.value(),
            };
            self.state.patients.insert(pid, patient);
            self.state.waiting_patients.push(pid);
            self.event_log.push(SimEvent::patient_arrived(tick, pid));
        }
    }

    fn run_triage(&mut self, tick: crate::time::tick::Tick) {
        let waiting = std::mem::take(&mut self.state.waiting_patients);
        let mut still_waiting = Vec::new();
        for pid in waiting {
            let patient = match self.state.patients.get(&pid) {
                Some(p) => p.clone(),
                None => continue,
            };
            if let Some(room_id) = self.triage.route(pid, &patient, &self.state.rooms, &mut self.state.room_queues) {
                if let Some(p) = self.state.patients.get_mut(&pid) {
                    p.assigned_room = Some(room_id);
                }
                self.event_log.push(SimEvent::patient_assigned(tick, pid, room_id));
            } else {
                still_waiting.push(pid);
            }
        }
        self.state.waiting_patients = still_waiting;
    }

    fn run_rooms(&mut self, tick: crate::time::tick::Tick) {
        let treated_ids = RoomEngine::process_queues(&mut self.state.room_queues);
        for pid in treated_ids {
            self.event_log.push(SimEvent::patient_treated(tick, pid));
            let ps = if let Some(p) = self.state.patients.remove(&pid) {
                PatientState {
                    id: p.id.0,
                    name: p.name.clone(),
                    disease_name: p.disease.name.clone(),
                    tick_admitted: p.tick_admitted,
                    tick_treated: Some(tick.value()),
                    outcome: "treated".to_string(),
                }
            } else {
                continue;
            };
            self.state.treated_patients.push(ps);
            self.economy.on_patient_treated(&mut self.state.budget);
            ReputationEngine::on_patient_treated(&mut self.state.reputation);
            self.event_log.push(SimEvent::patient_discharged(tick, pid));
            self.event_log.push(SimEvent::budget_updated(tick, self.state.budget.balance));
            self.event_log.push(SimEvent::reputation_changed(tick, self.state.reputation.score));
        }
    }
}
