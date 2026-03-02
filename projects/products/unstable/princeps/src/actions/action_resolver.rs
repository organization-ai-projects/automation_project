use crate::actions::action::Action;
use crate::model::candidate::Candidate;
use crate::model::candidate_id::CandidateId;
use crate::model::voter_block::VoterBlock;
use rand::Rng;
use rand::rngs::StdRng;

pub struct ActionResolver;

impl ActionResolver {
    pub fn resolve(
        &self,
        action: &Action,
        actor_id: &CandidateId,
        candidates: &mut Vec<Candidate>,
        voter_blocks: &mut Vec<VoterBlock>,
        rng: &mut StdRng,
    ) {
        match action {
            Action::CampaignRally { target_block } => {
                let (charisma, id) = {
                    let actor = candidates.iter().find(|c| &c.id == actor_id);
                    match actor {
                        Some(a) => (a.charisma, a.id.clone()),
                        None => return,
                    }
                };
                let effectiveness = (charisma as f64 / 100.0) * rng.random_range(0.5f64..1.5);
                for block in voter_blocks.iter_mut() {
                    if block.id == *target_block {
                        let entry = block.support.entry(id.clone()).or_insert(0.0);
                        *entry = (*entry + 0.05 * effectiveness).min(1.0);
                    }
                }
                if let Some(actor) = candidates.iter_mut().find(|c| &c.id == actor_id) {
                    actor.volunteers = (actor.volunteers + rng.random_range(10i64..50)).min(500);
                }
            }
            Action::MediaAppearance => {
                if let Some(actor) = candidates.iter_mut().find(|c| &c.id == actor_id) {
                    let effect = (actor.charisma as f64 / 100.0) * rng.random_range(0.3f64..1.2);
                    actor.approval = (actor.approval + 0.02 * effect).min(1.0);
                    actor.media = (actor.media + rng.random_range(5i64..20)).min(200);
                }
            }
            Action::PolicyAnnouncement { topic, position } => {
                if let Some(actor) = candidates.iter_mut().find(|c| &c.id == actor_id) {
                    actor.positions.insert(topic.clone(), *position);
                    let effect = actor.competence as f64 / 100.0;
                    actor.approval = (actor.approval + 0.01 * effect).min(1.0);
                }
            }
            Action::AttackOpponent { target } => {
                let charisma = {
                    let actor = candidates.iter().find(|c| &c.id == actor_id);
                    match actor {
                        Some(a) => a.charisma,
                        None => return,
                    }
                };
                let success = rng.random_bool(charisma as f64 / 150.0);
                if let Some(actor) = candidates.iter_mut().find(|c| &c.id == actor_id) {
                    if success {
                        actor.approval = (actor.approval + 0.015).min(1.0);
                    } else {
                        actor.approval = (actor.approval - 0.01).max(0.0);
                    }
                }
                if success {
                    if let Some(tgt) = candidates.iter_mut().find(|c| &c.id == target) {
                        tgt.approval = (tgt.approval - 0.02).max(0.0);
                    }
                }
            }
            Action::DamageControl => {
                if let Some(actor) = candidates.iter_mut().find(|c| &c.id == actor_id) {
                    let recovery = (actor.integrity as f64 / 100.0) * rng.random_range(0.5f64..1.5);
                    actor.approval = (actor.approval + 0.03 * recovery).min(1.0);
                }
            }
            Action::Fundraising => {
                if let Some(actor) = candidates.iter_mut().find(|c| &c.id == actor_id) {
                    let raised = rng.random_range(50_000i64..200_000);
                    actor.money += raised;
                }
            }
        }
    }
}
