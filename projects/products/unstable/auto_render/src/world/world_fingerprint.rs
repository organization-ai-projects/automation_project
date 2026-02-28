use sha2::{Sha256, Digest};
use super::WorldState;

pub struct WorldFingerprint {
    pub hash: String,
}

impl WorldFingerprint {
    pub fn compute(world: &WorldState) -> WorldFingerprint {
        let mut hasher = Sha256::new();
        hasher.update(format!("tick:{}", world.tick_id));
        for (id, entity) in &world.entities {
            hasher.update(format!("entity:{}:{}", id.0, entity.name));
            let px = (entity.transform.position[0] * 1000.0).round() as i64;
            let py = (entity.transform.position[1] * 1000.0).round() as i64;
            let pz = (entity.transform.position[2] * 1000.0).round() as i64;
            hasher.update(format!("pos:{},{},{}", px, py, pz));
        }
        let cam = &world.camera;
        let cx = (cam.transform.position[0] * 1000.0).round() as i64;
        let cy = (cam.transform.position[1] * 1000.0).round() as i64;
        let cz = (cam.transform.position[2] * 1000.0).round() as i64;
        let fov = (cam.fov_deg * 1000.0).round() as i64;
        hasher.update(format!("cam:{},{},{},fov:{}", cx, cy, cz, fov));
        let result = hasher.finalize();
        WorldFingerprint {
            hash: hex::encode(result),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{WorldState, EntityId, WorldEntity, Transform};

    #[test]
    fn same_world_same_fingerprint() {
        let world = WorldState::new();
        let f1 = WorldFingerprint::compute(&world);
        let f2 = WorldFingerprint::compute(&world);
        assert_eq!(f1.hash, f2.hash);
    }

    #[test]
    fn different_worlds_different_fingerprints() {
        let world1 = WorldState::new();
        let mut world2 = WorldState::new();
        world2.entities.insert(EntityId(1), WorldEntity {
            id: EntityId(1),
            transform: Transform::default(),
            name: "entity1".to_string(),
        });
        let f1 = WorldFingerprint::compute(&world1);
        let f2 = WorldFingerprint::compute(&world2);
        assert_ne!(f1.hash, f2.hash);
    }

    #[test]
    fn fingerprint_is_hex_string() {
        let world = WorldState::new();
        let f = WorldFingerprint::compute(&world);
        assert_eq!(f.hash.len(), 64);
        assert!(f.hash.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
