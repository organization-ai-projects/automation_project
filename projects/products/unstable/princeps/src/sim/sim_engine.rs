use std::collections::BTreeMap;
use rand::{Rng, SeedableRng, rngs::StdRng};
use sha2::{Sha256, Digest};
use crate::actions::action::Action;
use crate::actions::action_resolver::ActionResolver;
use crate::debate::debate::Debate;
use crate::debate::debate_resolver::DebateResolver;
use crate::diagnostics::error::PrincepsError;
use crate::events::campaign_event::CampaignEvent;
use crate::events::event_deck::EventDeck;
use crate::model::candidate::Candidate;
use crate::model::candidate_id::CandidateId;
use crate::model::faction::Faction;
use crate::model::game_id::GameId;
use crate::model::voter_block::VoterBlock;
use crate::poll::poll_model::PollModel;
use crate::poll::poll_report::PollReport;
use crate::replay::replay_file::{ActionEntry, ReplayFile};
use crate::report::end_report::EndReport;
use crate::report::run_summary::RunSummary;
use crate::sim::day::Day;

pub struct SimEngine {
    pub game_id: GameId,
    pub seed: u64,
    rng: StdRng,
    pub candidates: Vec<Candidate>,
    pub voter_blocks: Vec<VoterBlock>,
    pub event_deck: EventDeck,
    pub factions: Vec<Faction>,
    pub day_logs: Vec<Day>,
    pub poll_reports: Vec<PollReport>,
    pub debates_held: Vec<Debate>,
    pub replay: ReplayFile,
    debate_days: Vec<u32>,
}

impl SimEngine {
    pub fn with_defaults(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        let days_placeholder = 0;
        Self {
            game_id: GameId::new(seed, days_placeholder),
            seed,
            rng,
            candidates: default_candidates(),
            voter_blocks: default_voter_blocks(),
            event_deck: default_event_deck(),
            factions: default_factions(),
            day_logs: Vec::new(),
            poll_reports: Vec::new(),
            debates_held: Vec::new(),
            replay: ReplayFile::new(seed, days_placeholder),
            debate_days: vec![15, 28],
        }
    }

    pub fn run(&mut self, days: u32) -> Result<EndReport, PrincepsError> {
        if self.candidates.is_empty() {
            return Err(PrincepsError::NoCandidates);
        }

        self.game_id = GameId::new(self.seed, days);
        self.replay.days = days;

        for day_num in 1..=days {
            let mut day = Day::new(day_num);

            // Draw one event per day (probabilistic)
            if self.rng.random_bool(0.7) {
                let event = {
                    let available: Vec<usize> = (0..self.event_deck.cards.len())
                        .filter(|i| !self.event_deck.drawn_indices.contains(i))
                        .collect();
                    if available.is_empty() {
                        None
                    } else {
                        let pick = self.rng.random_range(0..available.len());
                        let idx = available[pick];
                        self.event_deck.drawn_indices.push(idx);
                        self.replay.drawn_event_indices.push((day_num, idx));
                        Some(self.event_deck.cards[idx].clone())
                    }
                };
                if let Some(ev) = event {
                    apply_campaign_event(&ev, &mut self.candidates);
                    day.events.push(ev);
                }
            }

            // AI action for each candidate
            let candidate_ids: Vec<CandidateId> =
                self.candidates.iter().map(|c| c.id.clone()).collect();
            let block_ids: Vec<String> =
                self.voter_blocks.iter().map(|b| b.id.clone()).collect();

            for cid in &candidate_ids {
                let action = choose_ai_action(cid, &self.candidates, &block_ids, &mut self.rng);
                self.replay.actions.push(ActionEntry {
                    day: day_num,
                    candidate_id: cid.clone(),
                    action: action.clone(),
                });
                ActionResolver.resolve(
                    &action,
                    cid,
                    &mut self.candidates,
                    &mut self.voter_blocks,
                    &mut self.rng,
                );
            }

            // Debate on scheduled days
            if self.debate_days.contains(&day_num) && day_num <= days {
                let debate =
                    DebateResolver.resolve(day_num, &self.candidates, &mut self.rng);
                for (cid, delta) in &debate.outcomes {
                    if let Some(c) = self.candidates.iter_mut().find(|c| &c.id == cid) {
                        c.approval = (c.approval + delta).clamp(0.0, 1.0);
                    }
                }
                day.debate = Some(debate.clone());
                self.debates_held.push(debate);
            }

            // Poll every 7 days or on final day
            if day_num % 7 == 0 || day_num == days {
                let poll = PollModel::compute(day_num, &self.candidates, &self.voter_blocks);
                day.poll = Some(poll.clone());
                self.poll_reports.push(poll);
            }

            self.day_logs.push(day);
        }

        self.generate_report(days)
    }

    fn generate_report(&self, days: u32) -> Result<EndReport, PrincepsError> {
        let final_poll = self
            .poll_reports
            .last()
            .cloned()
            .unwrap_or_else(|| PollModel::compute(days, &self.candidates, &self.voter_blocks));

        let winner = final_poll
            .leader()
            .cloned()
            .ok_or_else(|| PrincepsError::Simulation("no winner determined".into()))?;

        let candidate_final_approvals: BTreeMap<CandidateId, f64> = self
            .candidates
            .iter()
            .map(|c| (c.id.clone(), c.approval))
            .collect();

        let run_summary = RunSummary {
            seed: self.seed,
            days,
            total_events: self.day_logs.iter().map(|d| d.events.len()).sum(),
            total_debates: self.debates_held.len(),
            total_polls: self.poll_reports.len(),
            candidate_final_approvals,
        };

        // Build canonical JSON for hashing (excluding run_hash field)
        let hash_payload = serde_json::json!({
            "game_id": self.game_id,
            "winner": winner,
            "final_poll": final_poll,
            "run_summary": run_summary,
        });
        let canonical = serde_json::to_string(&hash_payload)
            .map_err(|e| PrincepsError::Serialization(e.to_string()))?;
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        let run_hash = hex::encode(hasher.finalize());

        Ok(EndReport {
            game_id: self.game_id.clone(),
            winner,
            final_poll,
            run_summary,
            run_hash,
        })
    }
}

fn apply_campaign_event(event: &CampaignEvent, candidates: &mut Vec<Candidate>) {
    if let Some(target_id) = event.target_candidate() {
        let delta = event.approval_delta();
        if let Some(c) = candidates.iter_mut().find(|c| &c.id == target_id) {
            c.approval = (c.approval + delta).clamp(0.0, 1.0);
        }
    }
}

fn choose_ai_action(
    actor_id: &CandidateId,
    candidates: &[Candidate],
    block_ids: &[String],
    rng: &mut StdRng,
) -> Action {
    let actor = match candidates.iter().find(|c| &c.id == actor_id) {
        Some(a) => a,
        None => return Action::Fundraising,
    };

    // If money is low → fundraise
    if actor.money < 100_000 {
        return Action::Fundraising;
    }

    // If approval is low → damage control
    if actor.approval < 0.15 {
        return Action::DamageControl;
    }

    // Weighted random choice based on candidate traits
    let choice = rng.random_range(0u32..6);
    match choice {
        0 => {
            let target_block = if block_ids.is_empty() {
                "general".to_string()
            } else {
                block_ids[rng.random_range(0..block_ids.len())].clone()
            };
            Action::CampaignRally { target_block }
        }
        1 => Action::MediaAppearance,
        2 => {
            let topics = ["economy", "healthcare", "environment", "security", "education"];
            let topic = topics[rng.random_range(0..topics.len())].to_string();
            let position = rng.random_range(-5i32..=5);
            Action::PolicyAnnouncement { topic, position }
        }
        3 => {
            let others: Vec<&Candidate> =
                candidates.iter().filter(|c| &c.id != actor_id).collect();
            if others.is_empty() {
                Action::MediaAppearance
            } else {
                let idx = rng.random_range(0..others.len());
                Action::AttackOpponent {
                    target: others[idx].id.clone(),
                }
            }
        }
        4 => Action::DamageControl,
        _ => Action::Fundraising,
    }
}

fn default_candidates() -> Vec<Candidate> {
    let mut c1 = Candidate::new("populiste", "Marcel Populiste", 85, 45, 30, 90);
    c1.positions.insert("economy".to_string(), -3);
    c1.positions.insert("security".to_string(), 4);

    let mut c2 = Candidate::new("technocrate", "Élise Technocrate", 40, 90, 75, 20);
    c2.positions.insert("economy".to_string(), 2);
    c2.positions.insert("healthcare".to_string(), 3);

    let mut c3 = Candidate::new("vert", "Jean-Paul Vert", 60, 65, 80, 40);
    c3.positions.insert("environment".to_string(), 5);
    c3.positions.insert("economy".to_string(), 1);

    let mut c4 = Candidate::new("conservateur", "Hortense Conservateur", 55, 70, 60, 35);
    c4.positions.insert("security".to_string(), 3);
    c4.positions.insert("economy".to_string(), -1);

    vec![c1, c2, c3, c4]
}

fn default_voter_blocks() -> Vec<VoterBlock> {
    let mut suburbanites = VoterBlock::new("suburbanites", "Banlieusards", 25);
    suburbanites.preferences.insert("economy".to_string(), -1);
    suburbanites.preferences.insert("security".to_string(), 2);
    suburbanites.sensitivities.insert("charisma".to_string(), 0.8);
    suburbanites.sensitivities.insert("economy".to_string(), 1.2);

    let mut young = VoterBlock::new("young_voters", "Jeunes Électeurs", 20);
    young.preferences.insert("environment".to_string(), 4);
    young.preferences.insert("economy".to_string(), 2);
    young.sensitivities.insert("charisma".to_string(), 1.5);
    young.sensitivities.insert("environment".to_string(), 2.0);

    let mut retirees = VoterBlock::new("retirees", "Retraités", 25);
    retirees.preferences.insert("healthcare".to_string(), 3);
    retirees.preferences.insert("security".to_string(), 3);
    retirees.sensitivities.insert("integrity".to_string(), 1.5);
    retirees.sensitivities.insert("healthcare".to_string(), 1.8);

    let mut workers = VoterBlock::new("workers", "Travailleurs", 20);
    workers.preferences.insert("economy".to_string(), -2);
    workers.preferences.insert("education".to_string(), 2);
    workers.sensitivities.insert("economy".to_string(), 2.0);
    workers.sensitivities.insert("charisma".to_string(), 0.6);

    let mut students = VoterBlock::new("students", "Étudiants", 10);
    students.preferences.insert("education".to_string(), 4);
    students.preferences.insert("environment".to_string(), 3);
    students.sensitivities.insert("charisma".to_string(), 1.2);
    students.sensitivities.insert("education".to_string(), 2.5);

    vec![suburbanites, young, retirees, workers, students]
}

fn default_event_deck() -> EventDeck {
    let cards = vec![
        CampaignEvent::Scandal {
            target: CandidateId::new("populiste"),
            description: "Leaked audio reveals off-colour joke about retirees.".to_string(),
            severity: 7,
            approval_delta: -0.08,
        },
        CampaignEvent::Scandal {
            target: CandidateId::new("technocrate"),
            description: "Expenses scandal: €12k dinner billed to campaign.".to_string(),
            severity: 5,
            approval_delta: -0.05,
        },
        CampaignEvent::Endorsement {
            target: CandidateId::new("vert"),
            source: "Confederation of Cyclists".to_string(),
            approval_delta: 0.04,
        },
        CampaignEvent::Endorsement {
            target: CandidateId::new("technocrate"),
            source: "Association of Economists".to_string(),
            approval_delta: 0.05,
        },
        CampaignEvent::Gaffe {
            target: CandidateId::new("populiste"),
            description: "Mispronounces the name of the capital three times live on TV.".to_string(),
            approval_delta: -0.04,
        },
        CampaignEvent::Gaffe {
            target: CandidateId::new("conservateur"),
            description: "Accidentally endorses competitor's slogan.".to_string(),
            approval_delta: -0.03,
        },
        CampaignEvent::PolicyWin {
            target: CandidateId::new("vert"),
            topic: "environment".to_string(),
            approval_delta: 0.06,
        },
        CampaignEvent::PolicyWin {
            target: CandidateId::new("technocrate"),
            topic: "healthcare".to_string(),
            approval_delta: 0.05,
        },
        CampaignEvent::PolicyFail {
            target: CandidateId::new("populiste"),
            topic: "economy".to_string(),
            approval_delta: -0.06,
        },
        CampaignEvent::PolicyFail {
            target: CandidateId::new("conservateur"),
            topic: "security".to_string(),
            approval_delta: -0.04,
        },
        CampaignEvent::Scandal {
            target: CandidateId::new("vert"),
            description: "Report: green candidate's campaign bus runs on diesel.".to_string(),
            severity: 6,
            approval_delta: -0.07,
        },
        CampaignEvent::Endorsement {
            target: CandidateId::new("conservateur"),
            source: "Rural Business Federation".to_string(),
            approval_delta: 0.04,
        },
        CampaignEvent::Gaffe {
            target: CandidateId::new("technocrate"),
            description: "Caught checking phone during veterans' memorial minute of silence.".to_string(),
            approval_delta: -0.06,
        },
        CampaignEvent::PolicyWin {
            target: CandidateId::new("populiste"),
            topic: "security".to_string(),
            approval_delta: 0.05,
        },
        CampaignEvent::Endorsement {
            target: CandidateId::new("populiste"),
            source: "National Taxi Drivers Union".to_string(),
            approval_delta: 0.03,
        },
        CampaignEvent::PolicyFail {
            target: CandidateId::new("vert"),
            topic: "economy".to_string(),
            approval_delta: -0.03,
        },
        CampaignEvent::Scandal {
            target: CandidateId::new("conservateur"),
            description: "Plagiarism accusation: speech lifted from 1998 local politician.".to_string(),
            severity: 4,
            approval_delta: -0.04,
        },
        CampaignEvent::PolicyWin {
            target: CandidateId::new("conservateur"),
            topic: "education".to_string(),
            approval_delta: 0.04,
        },
        CampaignEvent::Gaffe {
            target: CandidateId::new("vert"),
            description: "Forgets the name of the largest river during geography quiz show.".to_string(),
            approval_delta: -0.02,
        },
        CampaignEvent::PolicyFail {
            target: CandidateId::new("technocrate"),
            topic: "environment".to_string(),
            approval_delta: -0.05,
        },
    ];
    EventDeck::new(cards)
}

fn default_factions() -> Vec<Faction> {
    let mut left = Faction::new("left", "Coalition Progressiste");
    left.candidates.push(CandidateId::new("vert"));
    let mut center = Faction::new("center", "Alliance du Centre");
    center.candidates.push(CandidateId::new("technocrate"));
    let mut right = Faction::new("right", "Rassemblement National Modéré");
    right.candidates.push(CandidateId::new("conservateur"));
    let mut populist = Faction::new("populist", "Mouvement Populiste");
    populist.candidates.push(CandidateId::new("populiste"));
    vec![left, center, right, populist]
}
