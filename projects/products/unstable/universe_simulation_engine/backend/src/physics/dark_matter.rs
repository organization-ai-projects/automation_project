use crate::math::constants::GRAVITATIONAL_CONSTANT;
use crate::math::vec3::Vec3;
use crate::structures::galaxy::Galaxy;

pub struct DarkMatterEngine;

impl DarkMatterEngine {
    pub fn halo_gravitational_force(halo_mass: f64, obj_mass: f64, displacement: &Vec3) -> Vec3 {
        let r_sq = displacement.length_squared();
        if r_sq < 1e-20 {
            return Vec3::zero();
        }
        let magnitude = GRAVITATIONAL_CONSTANT * halo_mass * obj_mass / r_sq;
        displacement.normalized().scale(-magnitude)
    }

    pub fn apply_to_galaxies(galaxies: &mut [Galaxy], dt: f64) {
        let len = galaxies.len();
        if len < 2 {
            return;
        }
        let data: Vec<(Vec3, f64, f64)> = galaxies
            .iter()
            .map(|g| (g.position, g.dark_matter_halo_mass, g.total_mass()))
            .collect();

        for i in 0..len {
            let mut force = Vec3::zero();
            for j in 0..len {
                if i == j {
                    continue;
                }
                let displacement = data[j].0 - data[i].0;
                force += Self::halo_gravitational_force(data[j].1, data[i].2, &displacement);
            }
            if data[i].2 > 1e-50 {
                let accel = force.scale(1.0 / data[i].2);
                galaxies[i].velocity += accel.scale(dt);
                galaxies[i].position += galaxies[i].velocity.scale(dt);
            }
        }
    }
}
