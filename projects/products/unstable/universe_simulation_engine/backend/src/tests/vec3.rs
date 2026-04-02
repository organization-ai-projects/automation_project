#[test]
fn vec3_new() {
    let v = crate::math::vec3::Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn vec3_zero() {
    let v = crate::math::vec3::Vec3::zero();
    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
}

#[test]
fn vec3_length() {
    let v = crate::math::vec3::Vec3::new(3.0, 4.0, 0.0);
    assert!((v.length() - 5.0).abs() < 1e-10);
}

#[test]
fn vec3_normalized() {
    let v = crate::math::vec3::Vec3::new(0.0, 0.0, 5.0);
    let n = v.normalized();
    assert!((n.length() - 1.0).abs() < 1e-10);
    assert!((n.z - 1.0).abs() < 1e-10);
}

#[test]
fn vec3_dot() {
    let a = crate::math::vec3::Vec3::new(1.0, 0.0, 0.0);
    let b = crate::math::vec3::Vec3::new(0.0, 1.0, 0.0);
    assert!((a.dot(&b)).abs() < 1e-10);
}

#[test]
fn vec3_cross() {
    let a = crate::math::vec3::Vec3::new(1.0, 0.0, 0.0);
    let b = crate::math::vec3::Vec3::new(0.0, 1.0, 0.0);
    let c = a.cross(&b);
    assert!((c.z - 1.0).abs() < 1e-10);
}

#[test]
fn vec3_scale() {
    let v = crate::math::vec3::Vec3::new(1.0, 2.0, 3.0);
    let s = v.scale(2.0);
    assert_eq!(s.x, 2.0);
    assert_eq!(s.y, 4.0);
    assert_eq!(s.z, 6.0);
}

#[test]
fn vec3_distance() {
    let a = crate::math::vec3::Vec3::new(0.0, 0.0, 0.0);
    let b = crate::math::vec3::Vec3::new(3.0, 4.0, 0.0);
    assert!((a.distance_to(&b) - 5.0).abs() < 1e-10);
}

#[test]
fn vec3_add_operator() {
    let a = crate::math::vec3::Vec3::new(1.0, 2.0, 3.0);
    let b = crate::math::vec3::Vec3::new(4.0, 5.0, 6.0);
    let c = a + b;
    assert_eq!(c.x, 5.0);
    assert_eq!(c.y, 7.0);
    assert_eq!(c.z, 9.0);
}

#[test]
fn vec3_sub_operator() {
    let a = crate::math::vec3::Vec3::new(4.0, 5.0, 6.0);
    let b = crate::math::vec3::Vec3::new(1.0, 2.0, 3.0);
    let c = a - b;
    assert_eq!(c.x, 3.0);
    assert_eq!(c.y, 3.0);
    assert_eq!(c.z, 3.0);
}

#[test]
fn vec3_mul_operator() {
    let a = crate::math::vec3::Vec3::new(1.0, 2.0, 3.0);
    let c = a * 3.0;
    assert_eq!(c.x, 3.0);
    assert_eq!(c.y, 6.0);
    assert_eq!(c.z, 9.0);
}
