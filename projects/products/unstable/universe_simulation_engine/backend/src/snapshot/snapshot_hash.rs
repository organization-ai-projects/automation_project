use crate::snapshot::state_snapshot::StateSnapshot;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct SnapshotHash(pub String);

impl SnapshotHash {
    pub fn compute(snapshot: &StateSnapshot) -> Self {
        let state = &snapshot.state;

        let particles_summary: String = state
            .particles
            .iter()
            .filter(|p| p.alive)
            .map(|p| {
                format!(
                    "{}:{:?}:{:.6}:{:.6}:{:.6}",
                    p.id.0, p.kind, p.position.x, p.position.y, p.position.z
                )
            })
            .collect::<Vec<_>>()
            .join("|");

        let stars_summary: String = state
            .stars
            .iter()
            .map(|s| {
                format!(
                    "{}:{:?}:{:.6}:{}:{}",
                    s.id.0, s.class, s.mass, s.age_ticks, s.alive
                )
            })
            .collect::<Vec<_>>()
            .join("|");

        let galaxies_summary: String = state
            .galaxies
            .iter()
            .map(|g| {
                format!(
                    "{}:{:?}:{:.6}:{}",
                    g.id.0, g.galaxy_type, g.mass, g.age_ticks
                )
            })
            .collect::<Vec<_>>()
            .join("|");

        let canonical = format!(
            "tick={}#era={:?}#progress={:.6}#particles={}#stars={}#galaxies={}#filaments={}#voids={}",
            snapshot.tick.value(),
            state.era,
            state.era_progress,
            particles_summary,
            stars_summary,
            galaxies_summary,
            state.cosmic_web.filaments.len(),
            state.cosmic_web.voids.len(),
        );

        let hash = Sha256::digest(canonical.as_bytes());
        Self(hex::encode(hash))
    }
}
