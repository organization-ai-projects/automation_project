use super::WorldState;
use sha2::{Digest, Sha256};

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
            for (key, value) in &entity.components {
                hasher.update(format!("comp:{}={}", key, value));
            }
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
