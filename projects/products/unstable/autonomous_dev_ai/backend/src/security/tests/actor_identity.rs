//! projects/products/unstable/autonomous_dev_ai/src/security/tests/actor_identity.rs
use super::*;

#[test]
fn test_default_actor_identity() {
    let actor = ActorIdentity::default();
    assert_eq!(actor.id.to_string(), "autonomous_dev_ai");
    assert_eq!(actor.run_id.to_string(), "default_run");
    assert!(actor.roles.contains(&ActorRole::Developer));
}
