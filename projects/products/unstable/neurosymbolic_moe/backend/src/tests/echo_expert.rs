use crate::echo_expert::EchoExpert;
use crate::moe_core::ExpertCapability;

#[test]
fn echo_expert_constructor_is_wired() {
    let ctor: fn(&str, &str, Vec<ExpertCapability>) -> EchoExpert = EchoExpert::new;
    let _ = ctor;
}
