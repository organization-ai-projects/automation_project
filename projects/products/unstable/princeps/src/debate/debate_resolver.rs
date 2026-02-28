use crate::debate::debate::Debate;
use crate::model::candidate::Candidate;
use crate::model::candidate_id::CandidateId;
use rand::Rng;
use rand::rngs::StdRng;
use std::collections::BTreeMap;

pub struct DebateResolver;

impl DebateResolver {
    pub fn resolve(&self, day: u32, candidates: &[Candidate], rng: &mut StdRng) -> Debate {
        let participants: Vec<CandidateId> = candidates.iter().map(|c| c.id.clone()).collect();
        let mut transcript = Vec::new();
        let mut outcomes: BTreeMap<CandidateId, f64> = BTreeMap::new();

        transcript.push(format!(
            "=== Day {day} Presidential Debate — Opening Statements ==="
        ));

        for candidate in candidates {
            let performance = (candidate.charisma as f64 / 100.0)
                * (1.0 + (candidate.competence as f64 - 50.0) / 200.0)
                * rng.random_range(0.6f64..1.4);

            let moment = if performance > 1.0 {
                format!(
                    "{} delivers a brilliant line on economic justice. (crowd applauds)",
                    candidate.name
                )
            } else if performance > 0.7 {
                format!(
                    "{} gives a solid, measured answer on healthcare.",
                    candidate.name
                )
            } else {
                format!(
                    "{} visibly stumbles on a question about foreign policy.",
                    candidate.name
                )
            };

            transcript.push(moment);

            let delta = (performance - 0.8) * 0.08;
            outcomes.insert(candidate.id.clone(), delta);
        }

        transcript.push("=== Debate concluded. ===".to_string());

        Debate {
            day,
            participants,
            transcript,
            outcomes,
        }
    }
}
