use crate::math::vec3::Vec3;
use crate::physics::electromagnetism::ElectromagnetismEngine;
use crate::physics::gravity::GravityEngine;

#[test]
fn gravity_force_computation() {
    let m1 = 1e10;
    let m2 = 1e10;
    let displacement = Vec3::new(1e3, 0.0, 0.0);
    let force = GravityEngine::compute_force(m1, m2, &displacement);
    assert!(force.length() > 0.0);
    // Force should point toward the other mass (negative x)
    assert!(force.x < 0.0);
}

#[test]
fn em_force_computation() {
    let q1 = 1.6e-19;
    let q2 = 1.6e-19;
    let displacement = Vec3::new(1e-10, 0.0, 0.0);
    let force = ElectromagnetismEngine::compute_force(q1, q2, &displacement);
    assert!(force.length() > 0.0);
    // Same charges repel (positive direction)
    assert!(force.x > 0.0);
}

#[test]
fn gravity_zero_distance_safety() {
    let force = GravityEngine::compute_force(1.0, 1.0, &Vec3::zero());
    assert_eq!(force.length(), 0.0);
}

#[test]
fn em_zero_distance_safety() {
    let force = ElectromagnetismEngine::compute_force(1.0, 1.0, &Vec3::zero());
    assert_eq!(force.length(), 0.0);
}
